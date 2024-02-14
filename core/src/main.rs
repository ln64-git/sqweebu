// region: --- imports
mod _utils;
use actix_web::{web, App, HttpServer, Responder};
use core::PlaybackCommand;
use core::PlaybackManager;
use core::_utils::azure::speak_text;
use core::_utils::ollama::speak_ollama;
use core::{AppState, _utils::playback};
use std::sync::Arc;
use tokio::sync::Mutex;
// endregion: --- imports

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();
    let nexus = AppState {
        running: None,
        playback_send: playback::init_playback_channel().await,
    };

    let nexus_lock = Arc::new(Mutex::new(nexus));

    let playback_sender_clone = {
        let state = nexus_lock.lock().await;
        state.playback_send.clone()
    };

    let _ = speak_ollama(
        "What does the name Luke represent?".to_owned(),
        playback_sender_clone,
    )
    .await;

    println!("MAIN - Running main");
}
