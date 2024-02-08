//  src/_utils/server.rs

pub use crate::_utils::endpoints::playback_pause_endpoint;
pub use crate::_utils::endpoints::playback_resume_endpoint;
pub use crate::_utils::endpoints::playback_stop_endpoint;
pub use crate::_utils::endpoints::speak_clipboard_endpoint;
pub use crate::_utils::endpoints::speak_ollama_endpoint;
use crate::test_endpoint;
use crate::{AppState, AudioPlaybackManager, PlaybackCommand};
use actix_web::{web, App, HttpServer};
use response_engine::AudioRecordingManager;
use response_engine::RecordingCommand;
use std::sync::Mutex;
use tokio::runtime::Runtime;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Receiver;

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
    // Spawn the Queued Playback Thread
    let (record_tx, record_rx) = mpsc::channel::<RecordingCommand>(32);
    tokio::spawn(async move {
        recording_thread(record_rx).await;
    });

    // Spawn the Playback Control Thread
    let (playback_tx, playback_rx) = mpsc::channel::<PlaybackCommand>(32);
    let (queue_tx, queue_rx) = mpsc::channel::<PlaybackCommand>(32);
    // Spawn the Playback Control Thread
    tokio::spawn(async move {
        playback_playback_thread(playback_rx, queue_tx.clone()).await;
    });
    // Spawn the Queued Playback Thread
    std::thread::spawn(move || {
        queued_playback_thread(queue_rx);
    });

    // Server setup and start
    HttpServer::new(move || {
        let app_state = AppState {
            playback_tx: playback_tx.clone(),
            record_tx: record_tx.clone(),
        };

        App::new()
            .app_data(web::Data::new(Mutex::new(app_state)))
            .configure(register_endpoints)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn playback_playback_thread(
    mut playback_rx: mpsc::Receiver<PlaybackCommand>,
    queue_tx: mpsc::Sender<PlaybackCommand>,
) {
    while let Some(command) = playback_rx.recv().await {
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
