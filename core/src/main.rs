// region: --- imports
mod _utils;
use actix_web::{web, App, HttpServer, Responder};
use core::PlaybackCommand;
use core::PlaybackManager;
use core::_utils::azure::speak_text;
use core::_utils::ollama::speak_ollama;
use core::{AppState, _utils::playback};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
// endregion: --- imports

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();
    let nexus = AppState {
        playback_send: playback::init_playback_channel().await,
        sentence_map: Arc::new(Mutex::new(HashMap::new())),
    };

    println!("MAIN - Running main");
}
