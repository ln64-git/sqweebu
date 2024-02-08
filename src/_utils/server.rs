//  src/_utils/server.rs

#![allow(dead_code)]

pub use crate::_utils::endpoints::playback_pause_endpoint;
pub use crate::_utils::endpoints::playback_resume_endpoint;
pub use crate::_utils::endpoints::playback_stop_endpoint;
pub use crate::_utils::endpoints::speak_clipboard_endpoint;
pub use crate::_utils::endpoints::speak_ollama_endpoint;
use crate::test_endpoint;
use crate::{AppState, AudioPlaybackManager, PlaybackCommand};
use actix_web::{web, App, HttpServer};
use std::sync::Mutex;
use tokio::runtime::Runtime;
use tokio::sync::mpsc;

use super::endpoints::record_start_endpoint;
use super::endpoints::record_stop_endpoint;

fn register_endpoints(cfg: &mut web::ServiceConfig) {
    cfg.route("/speak_clipboard", web::get().to(speak_clipboard_endpoint))
        .route("/speak_ollama", web::post().to(speak_ollama_endpoint))
        .route("/playback/pause", web::get().to(playback_pause_endpoint))
        .route("/playback/stop", web::get().to(playback_stop_endpoint))
        .route("/playback/resume", web::get().to(playback_resume_endpoint))
        .route("/record/start", web::get().to(record_start_endpoint))
        .route("/record/stop", web::get().to(record_stop_endpoint))
        .route("/test", web::get().to(test_endpoint));
}

pub async fn launch_playback_server() -> std::io::Result<()> {
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
