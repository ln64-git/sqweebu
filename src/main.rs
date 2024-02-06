// src/main.rs

// region: --- mod
mod _utils;
// endregion: --- mod

// region: --- crates
pub use crate::_utils::azure::azure_response_to_audio;
pub use crate::_utils::azure::get_azure_response;
pub use crate::_utils::clipboard::get_clipboard;
pub use crate::_utils::clipboard::speak_clipboard;
pub use crate::_utils::endpoints::pause_playback_endpoint;
pub use crate::_utils::endpoints::resume_playback_endpoint;
pub use crate::_utils::endpoints::speak_clipboard_endpoint;
pub use crate::_utils::endpoints::speak_ollama_endpoint;
pub use crate::_utils::endpoints::stop_playback_endpoint;
pub use crate::_utils::ollama::ollama_generate_api;
pub use crate::_utils::ollama::speak_ollama;
// endregion: --- crates

// region: --- modules
use actix_web::{web, App, HttpServer};
use response_engine::{AppState, AudioPlaybackManager, PlaybackCommand};
use std::sync::Mutex;
use tokio::runtime::Runtime;
use tokio::sync::mpsc;
// endregion: --- modules

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    launch_playback_server().await
}

fn register_endpoints(cfg: &mut web::ServiceConfig) {
    cfg.route("/speak_clipboard", web::get().to(speak_clipboard_endpoint))
        .route("/speak_ollama", web::post().to(speak_ollama_endpoint))
        .route("/pause", web::get().to(pause_playback_endpoint))
        .route("/stop", web::get().to(stop_playback_endpoint))
        .route("/resume", web::get().to(resume_playback_endpoint));
}

async fn launch_playback_server() -> std::io::Result<()> {
    let (control_tx, control_rx) = mpsc::channel::<PlaybackCommand>(32);
    let (queue_tx, queue_rx) = mpsc::channel::<PlaybackCommand>(32);
    // Spawn the Playback Control Thread
    tokio::spawn(async move {
        playback_control_thread(control_rx, queue_tx.clone()).await;
    });
    // Spawn the Queued Playback Thread
    std::thread::spawn(move || {
        queued_playback_thread(queue_rx);
    });
    // Server setup and start
    HttpServer::new(move || {
        let app_state = AppState {
            control_tx: control_tx.clone(),
        };

        App::new()
            .app_data(web::Data::new(Mutex::new(app_state)))
            .configure(register_endpoints)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn playback_control_thread(
    mut control_rx: mpsc::Receiver<PlaybackCommand>,
    queue_tx: mpsc::Sender<PlaybackCommand>,
) {
    while let Some(command) = control_rx.recv().await {
        let _ = queue_tx.send(command).await;
    }
}

fn queued_playback_thread(mut queue_rx: mpsc::Receiver<PlaybackCommand>) {
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
}
