// region: --- imports
mod _utils;
use actix_web::{web, App, HttpServer, Responder};
use chrono::prelude::*;
use core::PlaybackCommand;
use core::PlaybackManager;
use core::_utils::azure::speak_text;
use core::_utils::ollama::speak_ollama;
use core::_utils::playback::ollama_playback_queue;
use core::{AppState, _utils::playback};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
// endregion: --- imports

#[tokio::main]
async fn main() {
    let start_time = Utc::now();
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    let nexus = AppState {
        playback_send: playback::init_playback_channel().await,
        sentence_map: Arc::new(Mutex::new(HashMap::new())), // Wrap HashMap in Arc<Mutex<>>
    };

    let nexus_lock = Arc::new(Mutex::new(nexus));
    let nexus_lock_clone = Arc::clone(&nexus_lock);

    speak_ollama(
        "list three things about yourself.".to_owned(),
        nexus_lock.clone(),
    )
    .await
    .unwrap_or_else(|e| eprintln!("Error in speak_ollama: {}", e));
    // let _ = speak_text(
    //     "Hello?",
    //     nexus_lock_clone.lock().await.playback_send.clone(),
    // )
    // .await;

    // speak_text("Hello?", nexus_lock).await;

    let end_time = Utc::now(); // Record end time
    let duration = end_time.signed_duration_since(start_time); // Calculate duration
    let seconds = duration.num_seconds(); // Extract seconds from duration
    println!("Execution time: {} seconds", seconds); // Print execution time
    println!("MAIN - Running main");
}
