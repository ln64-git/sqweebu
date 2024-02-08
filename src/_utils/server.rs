//  src/_utils/server.rs

use crate::AudioRecordingManager;
use crate::RecordingCommand;
pub use crate::_utils::endpoints::playback_pause_endpoint;
pub use crate::_utils::endpoints::playback_resume_endpoint;
pub use crate::_utils::endpoints::playback_stop_endpoint;
pub use crate::_utils::endpoints::speak_clipboard_endpoint;
pub use crate::_utils::endpoints::speak_ollama_endpoint;
use crate::test_endpoint;
use crate::AppState;
use actix_web::{web, App, HttpServer};
use std::sync::Mutex;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Receiver;

use super::endpoints::record_start_endpoint;
use super::endpoints::record_stop_endpoint;
use super::playback;

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
    // Spawn the Queued Playback Thread
    let (record_tx, record_rx) = mpsc::channel::<RecordingCommand>(32);
    tokio::spawn(async move {
        recording_thread(record_rx).await;
    });

    let playback_tx = playback::init_playback_channel().await;

    // Server setup and start
    HttpServer::new(move || {
        let app_state = AppState {
            playback_tx: playback_tx.clone(),
            record_tx: record_tx.clone(),
            transcribed_text: None,
        };

        App::new()
            .app_data(web::Data::new(Mutex::new(app_state)))
            .configure(register_endpoints)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn recording_thread(mut record_rx: Receiver<RecordingCommand>) {
    let recording_manager = AudioRecordingManager::new();

    while let Some(command) = record_rx.recv().await {
        match command {
            RecordingCommand::Start(_) => {
                // If start_recording is async, it should be awaited
                recording_manager
                    .start_recording()
                    .await
                    .expect("Failed to start recording");
            }
            RecordingCommand::Stop => {
                // If stop_recording is async, it should be awaited
                recording_manager
                    .stop_recording()
                    .await
                    .expect("Failed to stop recording");
            }
        }
    }
}
