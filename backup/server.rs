//  src/_utils/server.rs

// region: --- Dependencies ---
use super::endpoints::record_start_endpoint;
use super::endpoints::record_stop_endpoint;
pub use crate::_utils::endpoints::playback_pause_endpoint;
pub use crate::_utils::endpoints::playback_resume_endpoint;
pub use crate::_utils::endpoints::playback_stop_endpoint;
pub use crate::_utils::endpoints::speak_clipboard_endpoint;
pub use crate::_utils::endpoints::speak_ollama_endpoint;
use crate::test_endpoint;
use crate::AudioRecordingManager;
use crate::RecordingCommand;
use crate::{AppState, AudioPlaybackManager, PlaybackCommand};
use actix_web::{web, App, HttpServer};
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;
use tokio::runtime::Runtime;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Receiver;
// endregion: ---

// region: --- Main Thread ---

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
    let (record_tx, record_rx) = mpsc::channel::<RecordingCommand>(32);
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

    let app_state = Arc::new(Mutex::new(AppState {
        playback_tx: playback_tx.clone(),
        record_tx: record_tx.clone(),
        transcribed_text: None,
    }));

    tokio::spawn(async move {
        recording_thread(record_rx, app_state_clone).await;
    });

    // Server setup and start Main Thread
    HttpServer::new(move || {
        let app_state = AppState {
            playback_tx: playback_tx.clone(),
            record_tx: record_tx.clone(),
            transcribed_text: None, // Initial state of AppState
        };

        App::new()
            .app_data(web::Data::new(Mutex::new(app_state))) // Sharing AppState across handlers
            .configure(register_endpoints) // Registering your endpoints
    })
    .bind("127.0.0.1:8080")? // Binding server to localhost on port 8080
    .run()
    .await
}

// endregion: --- Main

// region: --- Plauyback Thread ---

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

async fn playback_playback_thread(
    mut playback_rx: mpsc::Receiver<PlaybackCommand>,
    queue_tx: mpsc::Sender<PlaybackCommand>,
) {
    while let Some(command) = playback_rx.recv().await {
        let _ = queue_tx.send(command).await;
    }
}

// endregion: --- Plau

// region: --- Recording Thread ---
async fn recording_thread(
    mut record_rx: Receiver<RecordingCommand>,
    app_state: Arc<Mutex<AppState>>, // Assume this is defined elsewhere correctly
) {
    let recording_manager = AudioRecordingManager::new();
    let mut last_temp_file_path: Option<PathBuf> = None; // To keep track of the last recording path

    while let Some(command) = record_rx.recv().await {
        match command {
            RecordingCommand::Start => {
                if let Err(e) = recording_manager.start_recording().await {
                    eprintln!("Failed to start recording: {}", e);
                }
            }
            RecordingCommand::Stop => {
                if let Some(temp_file_path) = last_temp_file_path.take() {
                    // Take the path to use and clear it
                    if let Err(e) = recording_manager
                        .stop_recording(&app_state, temp_file_path)
                        .await
                    {
                        eprintln!("Failed to stop recording: {}", e);
                    }
                } else {
                    eprintln!("No recording path available for stopping.");
                }
            }
        }
    }
}

// endregion: --- Rec
