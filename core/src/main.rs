// region: --- imports
mod _utils;
use actix_web::{web, App, HttpServer, Responder};
use core::PlaybackCommand;
use core::PlaybackManager;
use core::_utils::azure::speak_text;
use core::_utils::ollama::speak_ollama;
use core::_utils::playback::ollama_playback_queue;
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
        sentence_queue: Vec::new(),
    };

    let nexus_lock = Arc::new(Mutex::new(nexus));

    // Create a clone of nexus_lock to be used in the closure
    let nexus_lock_clone = Arc::clone(&nexus_lock);

    speak_ollama(
        "list three things about yourself.".to_owned(),
        nexus_lock.clone(),
    )
    .await
    .unwrap_or_else(|e| eprintln!("Error in speak_ollama: {}", e));


    println!("MAIN - Running main");
}
