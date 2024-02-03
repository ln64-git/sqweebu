// src/main.rs

mod _api;
mod _utils;

// region: --- crates
pub use crate::_api::azure::azure_response_to_audio;
pub use crate::_api::azure::get_azure_response;
pub use crate::_api::ollama::ollama_generate_api;
pub use crate::_api::ollama::speak_ollama;
pub use crate::_utils::audio::speak_text;
pub use crate::_utils::clipboard::get_clipboard;
pub use crate::_utils::clipboard::speak_clipboard;
pub use crate::_utils::endpoints::pause_audio_endpoint;
pub use crate::_utils::endpoints::resume_audio_endpoint;
pub use crate::_utils::endpoints::speak_clipboard_endpoint;
pub use crate::_utils::endpoints::speak_ollama_endpoint;
pub use crate::_utils::endpoints::stop_audio_endpoint;
// endregion: --- crates

// region: --- modules
use _utils::endpoints;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use response_engine::{AppState, AudioPlaybackManager, PlaybackCommand};
use std::sync::Mutex;
use std::thread;
use tokio::runtime::Runtime;
use tokio::sync::mpsc;
use tokio::task::LocalSet;
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
            .route(
                "/pause/{id}",
                web::post().to(endpoints::pause_audio_endpoint),
            )
            .route(
                "/resume/{id}",
                web::post().to(endpoints::resume_audio_endpoint),
            )
            .route("/stop/{id}", web::post().to(endpoints::stop_audio_endpoint))
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

fn queued_playback_thread(queue_rx: mpsc::Receiver<PlaybackCommand>) {
    thread::spawn(move || {
        let runtime = Runtime::new().unwrap(); // Create a new Tokio runtime in this thread
        runtime.block_on(async {
            let mut audio_manager = AudioPlaybackManager::new();

            let mut queue_rx = queue_rx; // Take ownership of queue_rx in this thread
            while let Some(command) = queue_rx.recv().await {
                match command {
                    PlaybackCommand::Play(audio_data) => {
                        let _ = audio_manager.play_audio(audio_data).await;
                    }
                    PlaybackCommand::Stop(id) => {
                        audio_manager.stop_audio(id);
                    }
                    PlaybackCommand::Pause(id) => {
                        audio_manager.pause_audio(id);
                    }
                    PlaybackCommand::Resume(id) => {
                        audio_manager.resume_audio(id);
                    }
                    _ => {}
                }
            }
        });
    });
}
