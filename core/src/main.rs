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
    // let nexus = Arc::new(Mutex::new(AppState {
    //     playback_send: playback::init_playback_channel().await,
    // }));
    // let playback_send = {
    //     let nexus_lock = nexus.lock().await;
    //     nexus_lock.playback_send.clone()
    // };

    // speak_text("hello?", &playback_send).await;
    // speak_ollama(
    //     "what is the color of night in the elder scrolls 4 obiviion".to_owned(),
    //     nexus,
    // );
    println!("MAIN - Running main");
}
