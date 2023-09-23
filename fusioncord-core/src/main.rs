use std::{env, error::Error, io, sync::mpsc};

use fusioncord_core::client::Client;
use tracing::{subscriber, Level};
use tracing_subscriber::FmtSubscriber;
use twilight_model::gateway::{
    payload::outgoing::identify::{IdentifyInfo, IdentifyProperties},
    Intents, ShardId,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    subscriber::set_global_default(subscriber)?;

    let token = env::var("TOKEN").unwrap_or_else(|_| {
        let mut buf = String::new();
        println!("Enter your discord token...");
        io::stdin().read_line(&mut buf).expect("IO error");
        buf
    });

    let identify = IdentifyInfo {
        compress: false,
        intents: Intents::all(),
        large_threshold: 50,
        presence: None,
        properties: IdentifyProperties::new("chrome", "web", "windows"),
        shard: Some(ShardId::ONE),
        token,
    };

    let client = Client::new().await?;

    // TODO: remove this file
    // tx should be noop to avoid panics, whats even the point of this?
    let (tx, _) = mpsc::channel();

    Ok(client
        .wait_for_hello()
        .await?
        .identify(identify)
        .await?
        .wait_for_ready(tx)
        .await?
        .run()
        .await?)
}
