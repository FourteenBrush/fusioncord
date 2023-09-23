use std::{env, error::Error, io, sync::mpsc};

use eframe::NativeOptions;
use fusioncord_core::client::Client;
use fusioncord_ui::app::Application;
use tokio::runtime::Builder;
use twilight_model::gateway::{
    payload::outgoing::identify::{IdentifyInfo, IdentifyProperties},
    Intents, ShardId,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // create an async runtime
    // spawn the renderer on another thread
    // transition between client states and let the renderer know we're changing state (e.g. login -> logged in)
    // client initialized -> sent initial data for renderer (friends list etc)
    // main loop

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

    let native_options = NativeOptions::default();

    let rt = Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .build()
        .unwrap();

    let _ = rt.enter();

    let (tx, rx) = mpsc::channel();

    rt.spawn(async move {
        let mut client = Client::new().await?
            .wait_for_hello().await?
            .identify(identify).await?
            .wait_for_ready(tx).await?;

        client.run().await
    });

    eframe::run_native(
        "app",
        native_options,
        Box::new(move |cc| Box::new(Application::new(cc, rx))),
    )?;

    Ok(())
}
