// src/main.rs

// region: --- mod
mod _api;
mod _utils;
// endregion: --- mod

// region: --- crates
pub use crate::_api::azure::azure_response_to_audio;
pub use crate::_api::azure::get_azure_response;
pub use crate::_api::ollama::ollama_generate_api;
pub use crate::_api::ollama::speak_ollama;
pub use crate::_utils::audio::speak_text;
pub use crate::_utils::clipboard::get_clipboard;
pub use crate::_utils::clipboard::speak_clipboard;
pub use crate::_utils::endpoints::speak_clipboard_endpoint;
pub use crate::_utils::endpoints::speak_ollama_endpoint;
// endregion: --- crates

// region: --- modules
use _utils::endpoints;
use actix_web::{web, App, HttpServer};
use response_engine::{AppState, AudioPlaybackManager, PlaybackCommand};
use std::sync::Mutex;
use std::thread;
use tokio::runtime::Runtime;
use tokio::sync::mpsc;
// endregion: --- modules

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let (tx, rx) = mpsc::channel::<PlaybackCommand>(32);
    let (queue_tx, queue_rx) = mpsc::channel::<PlaybackCommand>(32);

    // Correctly spawn the Playback Control Thread as it is async
    tokio::spawn(playback_control_thread(rx, queue_tx.clone()));

    // Directly call queued_playback_thread without tokio::spawn
    queued_playback_thread(queue_rx);
    // Server setup and start
    let server_future = HttpServer::new(move || {
        let app_state = AppState { tx: tx.clone() };

        App::new()
            .app_data(web::Data::new(Mutex::new(app_state)))
            .route(
                "/speak_clipboard",
                web::get().to(endpoints::speak_clipboard_endpoint),
            )
            .route(
                "/speak_ollama",
                web::post().to(endpoints::speak_ollama_endpoint),
            )
    })
    .bind("127.0.0.1:8080")?
    .run();

    server_future.await
}

// Playback Control Thread
async fn playback_control_thread(
    mut rx: mpsc::Receiver<PlaybackCommand>,
    queue_tx: mpsc::Sender<PlaybackCommand>,
) {
    while let Some(command) = rx.recv().await {
        // Forward commands to the third thread for queued playback
        let _ = queue_tx.send(command).await;
    }
}

fn queued_playback_thread(mut queue_rx: mpsc::Receiver<PlaybackCommand>) {
    thread::spawn(move || {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let mut audio_manager = AudioPlaybackManager::new();
            while let Some(command) = queue_rx.recv().await {
                audio_manager.command_queue.push_back(command);
                if audio_manager
                    .is_idle
                    .load(std::sync::atomic::Ordering::SeqCst)
                {
                    audio_manager
                        .is_idle
                        .store(false, std::sync::atomic::Ordering::SeqCst);
                    audio_manager.start_processing_commands().await;
                    audio_manager
                        .is_idle
                        .store(true, std::sync::atomic::Ordering::SeqCst);
                }
            }
        });
    });
}
