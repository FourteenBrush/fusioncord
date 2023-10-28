use futures_util::{SinkExt, StreamExt};
use serde::Serialize;
use tokio::net::TcpStream;
use tokio_tungstenite::{
    connect_async,
    tungstenite::{self, protocol::CloseFrame, Message as TungsteniteMessage},
    MaybeTlsStream, WebSocketStream,
};

const GATEWAY_URL: &str = "wss://gateway.discord.gg/?v=10&encoding=json";

#[derive(Debug)]
pub struct Connection {
    stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
}

impl Connection {
    pub async fn new() -> Result<Self, tungstenite::Error> {
        Ok(Self {
            stream: connect_async(GATEWAY_URL).await?.0,
        })
    }

    pub async fn read(&mut self) -> Result<Message, ReceiveError> {
        self.stream
            .next()
            .await
            .ok_or(ReceiveError::ConnectionClosed)?
            .map(Message::from_tungstenite)
            .map_err(ReceiveError::from)?
            .ok_or(ReceiveError::UnexpectedMessageType)
    }

    pub async fn send<S: Serialize>(&mut self, msg: S) -> Result<(), SendError> {
        let msg = TungsteniteMessage::Text(serde_json::to_string(&msg)?);
        Ok(self.stream.send(msg).await?)
    }
}

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub enum ReceiveError {
    #[error("connection was unexpectedly closed")]
    ConnectionClosed,
    #[error("unexpected message type")]
    UnexpectedMessageType,
    Transmission(#[from] tungstenite::Error),
}

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub enum SendError {
    Serialisation(#[from] serde_json::Error),
    Transmission(#[from] tungstenite::Error),
}

#[derive(Debug)]
pub enum Message {
    Text(String),
    Close(Option<CloseFrame<'static>>),
}

impl Message {
    fn from_tungstenite(msg: TungsteniteMessage) -> Option<Self> {
        Some(match msg {
            TungsteniteMessage::Text(txt) => Self::Text(txt),
            TungsteniteMessage::Close(close_frame) => Self::Close(close_frame),
            _ => return None,
        })
    }
}

impl From<Message> for TungsteniteMessage {
    fn from(value: Message) -> Self {
        match value {
            Message::Text(txt) => TungsteniteMessage::Text(txt),
            Message::Close(close_frame) => TungsteniteMessage::Close(close_frame),
        }
    }
}
