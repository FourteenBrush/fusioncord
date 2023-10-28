use core::fmt;
use std::{
    fs,
    ops::{Deref, DerefMut},
    sync::{
        atomic::{AtomicBool, AtomicU16, Ordering},
        mpsc::Sender,
        Arc,
    },
    time::{Duration, Instant},
};

use serde::de::DeserializeSeed;
use serde_json::{Deserializer, Map, Value};
use tokio::{select, time::interval};
use tokio_tungstenite::tungstenite::{self, protocol::CloseFrame};
use tracing::{error, info, trace, warn};
use twilight_model::gateway::{
    event::{DispatchEvent, GatewayEvent, GatewayEventDeserializer},
    payload::outgoing::{identify::IdentifyInfo, Heartbeat, Identify},
    CloseCode,
};

use crate::{
    connection::{Connection, Message, ReceiveError, SendError},
    message::RenderMessage,
};

/// A client state-machine.
/// State transitions:
/// WaitingForHello -> WaitingForIdentify -> WaitingForReady -> Initialized
#[derive(Debug)]
pub struct Client<S: ClientState> {
    connection: Connection,
    state: S,
}

impl<S: ClientState> Client<S> {
    fn with_state<Target: ClientState>(connection: Connection, state: Target) -> Client<Target> {
        Client { connection, state }
    }

    async fn deserialize_gateway_event(&mut self) -> Result<GatewayEvent, ClientError> {
        let json = match self.connection.read().await? {
            Message::Text(json) => json,
            _ => return Err(ClientError::UnexpectedMessageType),
        };

        // trace!("got here");
        GatewayEventDeserializer::from_json(&json)
            .expect("missing opcode")
            .deserialize(&mut Deserializer::from_str(&json))
            .map_err(|e| {
                error!("An error occurred while deserializing a payload: {e:#?}");
                ClientError::from(e)
            })
    }
}

impl<S: ClientState> Deref for Client<S> {
    type Target = S;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.state
    }
}

impl<S: ClientState> DerefMut for Client<S> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.state
    }
}

impl Client<WaitingForHello> {
    pub async fn new() -> Result<Self, ClientError> {
        Ok(Self::with_state(Connection::new().await?, WaitingForHello))
    }

    pub async fn wait_for_hello(mut self) -> Result<Client<WaitingForIdentify>, ClientError> {
        let event = self.deserialize_gateway_event().await?;

        if let GatewayEvent::Hello(payload) = event {
            return Ok(Self::with_state(
                self.connection,
                WaitingForIdentify {
                    heartbeat_interval: Duration::from_millis(payload.heartbeat_interval),
                },
            ));
        }

        Err(ClientError::NoHandshake)
    }
}

impl Client<WaitingForIdentify> {
    pub async fn identify(
        mut self,
        identify: IdentifyInfo,
    ) -> Result<Client<WaitingForReady>, ClientError> {
        trace!("Sending identify");
        self.connection.send(Identify::new(identify)).await?;

        Ok(Self::with_state(self.connection, self.state.into()))
    }
}

impl Client<WaitingForReady> {
    pub async fn wait_for_ready(
        mut self,
        _tx: Sender<RenderMessage>,
    ) -> Result<Client<Initialized>, ClientError> {
        let event = self.deserialize_gateway_event().await?;

        if let GatewayEvent::Dispatch(seq, DispatchEvent::Ready(_payload)) = event {
            return Ok(Self::with_state(
                self.connection,
                Initialized {
                    heartbeat_interval: self.state.heartbeat_interval,
                    last_heartbeat: Instant::now() - Duration::from_secs(10_000),
                    last_heartbeat_acked: true,
                    last_seq: seq,
                    client_specific_payloads: Map::new(),
                    interrupted: Arc::new(AtomicBool::new(false)),
                },
            ));
        }

        Err(ClientError::NoReady)
    }
}

impl Client<Initialized> {
    pub async fn run(&mut self) -> Result<(), ClientError> {
        let mut heartbeat_ticker = interval(self.heartbeat_interval);

        while !self.interrupted.load(Ordering::Relaxed) {
            select! {
                _ = heartbeat_ticker.tick() => {
                    self.send_heartbeat().await?;
                }
                message = self.connection.read() => {
                    match message.unwrap() {
                        // TODO: don't clone txt
                        Message::Text(txt) => if let Err(e) = self.handle_message(txt.clone()).await {
                            error!("{e:#?} payload: {txt:#}");
                        },
                        Message::Close(close_frame) => self.handle_gateway_close(close_frame).await,
                    };
                }
            }
        }
        Ok(())
    }

    async fn send_heartbeat(&mut self) -> Result<(), ClientError> {
        static HEARBEATS_NOT_ACKED: AtomicU16 = AtomicU16::new(0);
        if !self.last_heartbeat_acked {
            let old_amount = HEARBEATS_NOT_ACKED.fetch_add(1, Ordering::SeqCst);
            warn!("Last heartbeat was not acknowledged ({})", old_amount + 1);
            // TODO: docs state we should reconnect
        } else {
            HEARBEATS_NOT_ACKED.fetch_min(1, Ordering::SeqCst);
        }
        self.last_heartbeat_acked = false;

        let payload = Heartbeat::new(self.last_seq.into());
        self.connection.send(payload).await?;

        self.last_heartbeat = Instant::now();
        trace!("Sent heartbeat");
        Ok(())
    }

    async fn handle_message(&mut self, json: String) -> Result<(), ClientError> {
        let event = self.deserialize_gateway_event().await.map_err(|e| {
            let err_msg = e.to_string();
            if err_msg.contains("unknown variant") {
                let event_name = &err_msg["unknown variant".len()..err_msg.rfind('`').unwrap()];
                warn!("Unknown event variant (user specific?): {event_name}");

                let json = serde_json::from_str(&json).unwrap();
                self.client_specific_payloads
                    .insert(event_name.to_owned(), json);
                // unefficient but whatever, Drop impl doesn't work
                fs::write(
                    "client_payloads.json",
                    serde_json::to_string_pretty(&self.client_specific_payloads).unwrap(),
                )
                .unwrap();
            }
            e
        })?;

        match event {
            GatewayEvent::Heartbeat(_) => self.send_heartbeat().await?,
            GatewayEvent::HeartbeatAck => {
                self.last_heartbeat_acked = true;
                trace!("Last heartbeat was acknowlegded");
            }
            GatewayEvent::Dispatch(seq, event) => {
                self.handle_dispatch_event(event).await;
                self.last_seq = seq;
            }
            _ => (), //warn!("unhandled event {event:?}"),
        }

        Ok(())
    }

    async fn handle_gateway_close(&mut self, close_frame: Option<CloseFrame<'static>>) {
        let close_code = close_frame.and_then(|f| {
            let code = u16::from(f.code);
            CloseCode::try_from(code).ok()
        });

        if let Some(close_code) = close_code {
            warn!("Gateway closed with code {close_code}");
        }

        // TODO: reconnect
    }

    async fn handle_dispatch_event(&mut self, event: DispatchEvent) {
        match event {
            DispatchEvent::Ready(_ready) => {
                // TODO: initialize client state
                info!("Successfully received the Ready event");
            }
            _ => warn!("Unimplemented dispatch event {:?}", event.kind()),
        }
    }
}

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub enum ClientError {
    Tungstenite(#[from] tungstenite::Error),
    Json(#[from] serde_json::Error),
    SendError(#[from] SendError),
    ReceiveError(#[from] ReceiveError),
    #[error("gateway did not sent a Hello event")]
    NoHandshake,
    #[error("gateway did not sent initial data")]
    NoReady,
    #[error("connection was unexpectedly closed")]
    ConnectionClosed,
    #[error("unexpected message type")]
    UnexpectedMessageType,
}

mod private {
    pub trait Sealed {}
}

macro_rules! state_impl {
    ($($type:ty),*) => {
        $(
            impl private::Sealed for $type {}
            impl ClientState for $type {}
        )*
    };
}

state_impl!(
    WaitingForHello,
    WaitingForIdentify,
    WaitingForReady,
    Initialized
);

pub trait ClientState: private::Sealed + fmt::Debug {}

/// Initial client state, client is waiting for a Hello payload
/// to perform its initial handshake
#[derive(Debug)]
pub struct WaitingForHello;

/// Client has received a Hello payload and is now going to
/// sent a Identify payload
#[derive(Debug)]
pub struct WaitingForIdentify {
    heartbeat_interval: Duration,
}

/// Client has sent an Identify payload and is now waiting to
/// receive a Ready payload
#[derive(Debug)]
pub struct WaitingForReady {
    heartbeat_interval: Duration,
}

impl From<WaitingForIdentify> for WaitingForReady {
    fn from(value: WaitingForIdentify) -> Self {
        Self {
            heartbeat_interval: value.heartbeat_interval,
        }
    }
}

/// Client has received a Ready payload and has fully initialized its state
#[derive(Debug)]
pub struct Initialized {
    heartbeat_interval: Duration,
    last_heartbeat: Instant,
    last_heartbeat_acked: bool,
    last_seq: u64,
    client_specific_payloads: Map<String, Value>,
    interrupted: Arc<AtomicBool>,
}
