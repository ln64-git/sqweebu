// src/lib.rs

mod _utils;

// region: --- crates
pub use crate::_utils::azure::azure_response_to_audio;
pub use crate::_utils::azure::get_azure_response;
pub use crate::_utils::clipboard::get_clipboard;
pub use crate::_utils::clipboard::speak_clipboard;
pub use crate::_utils::endpoints::playback_pause_endpoint;
pub use crate::_utils::endpoints::playback_resume_endpoint;
pub use crate::_utils::endpoints::playback_stop_endpoint;
pub use crate::_utils::endpoints::speak_clipboard_endpoint;
pub use crate::_utils::endpoints::speak_ollama_endpoint;
pub use crate::_utils::ollama::ollama_generate_api;
pub use crate::_utils::ollama::speak_ollama;
pub use crate::_utils::record::record_audio;
pub use crate::_utils::server::launch_playback_server;
pub use crate::_utils::test::test_endpoint;
pub use crate::_utils::transcribe::speech_to_text;
// endregion: --- crates

use anyhow::anyhow;
// region: --- imports
use anyhow::Result;
use rodio::Decoder;
use rodio::OutputStream;
use rodio::Sink;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::error::Error;
use std::fs;
use std::io::Cursor;
use std::path::PathBuf;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::sync::Mutex;
use tempfile::NamedTempFile;
use tokio::sync::Notify;

// endregion: --- imports

use tokio::sync::mpsc::Sender;

pub struct AppState {
    pub playback_tx: Sender<PlaybackCommand>,
    pub record_tx: Sender<RecordingCommand>,
    pub transcribed_text: Option<String>,
}

// region: --- Recording Manager

pub enum RecordingCommand {
    Start,
    Stop,
}

struct AudioRecordingManager {
    is_recording: Arc<AtomicBool>,
    notify_stop: Arc<Notify>,
}

impl AudioRecordingManager {
    pub fn new() -> Self {
        Self {
            is_recording: Arc::new(AtomicBool::new(false)),
            notify_stop: Arc::new(Notify::new()),
        }
    }
    pub async fn start_recording(&self) -> Result<PathBuf> {
        if self.is_recording.swap(true, Ordering::SeqCst) {
            // If already recording, return an error
            println!("Recording is already in progress.");
            return Err(anyhow!("Recording is already in progress"));
        }

        // Generate a temporary file path for recording
        let temp_file = NamedTempFile::new()
            .map_err(|e| anyhow!("Failed to create a temporary file for recording: {}", e))?;
        let temp_file_path = temp_file.into_temp_path();

        println!("Recording started, file path: {}", temp_file_path.display());
        let notify_stop_clone = self.notify_stop.clone();

        // Clone `temp_file_path` for use inside the async block
        let temp_file_path_clone: PathBuf = temp_file_path.to_path_buf();

        let result = record_audio(temp_file_path_clone, notify_stop_clone).await;

        if let Err(e) = result {
            eprintln!("Error during recording: {}", e);
        }

        // Successfully return the original path to the temporary file
        Ok(temp_file_path.to_path_buf())
    }

    pub async fn stop_recording(
        &self,
        app_state: &Arc<Mutex<AppState>>,
        temp_file_path: PathBuf,
    ) -> Result<()> {
        if self.is_recording.load(Ordering::SeqCst) {
            println!("Stopping recording...");
            // Signal or perform actions necessary to stop the recording here
            self.is_recording.store(false, Ordering::SeqCst);

            // Process the recorded audio for speech-to-text conversion
            let text_result = speech_to_text(&temp_file_path).await;
            match text_result {
                Ok(text) => {
                    let mut app_state_guard = app_state.lock().unwrap();
                    app_state_guard.transcribed_text = Some(text);
                    println!("Transcription updated.");
                }
                Err(e) => eprintln!("Error converting speech to text: {}", e),
            }

            // Attempt to delete the temporary file
            if let Err(e) = fs::remove_file(&temp_file_path) {
                eprintln!("Error deleting temporary file: {}", e);
            } else {
                println!("Temporary file deleted successfully.");
            }

            Ok(())
        } else {
            println!("Recording is not currently active.");
            Err(anyhow!("Recording is not currently active"))
        }
    }
}

// endregion: --- Recording Manager

// region: --- Playback Manager

pub enum PlaybackCommand {
    Play(Vec<u8>),
    Pause,
    Stop,
    Resume,
}

type SinkId = usize;

pub struct AudioPlaybackManager {
    pub next_id: SinkId,
    pub sinks: HashMap<SinkId, Sink>,
    pub streams: HashMap<SinkId, OutputStream>,
    pub command_queue: VecDeque<PlaybackCommand>,
    pub is_idle: AtomicBool,
    pub current_sink: Option<SinkId>, // New field to track the current playing audio
}

impl AudioPlaybackManager {
    pub fn new() -> Self {
        AudioPlaybackManager {
            next_id: 0,
            sinks: HashMap::new(),
            streams: HashMap::new(),
            command_queue: VecDeque::new(),
            is_idle: AtomicBool::new(true),
            current_sink: None,
        }
    }

    pub async fn start_processing_commands(&mut self) {
        while let Some(command) = self.command_queue.pop_front() {
            self.handle_command(command)
                .await
                .expect("Failed to handle command");
        }
    }

    pub async fn handle_command(&mut self, command: PlaybackCommand) -> Result<(), Box<dyn Error>> {
        match command {
            PlaybackCommand::Play(audio_data) => {
                self.play_audio(audio_data).await?;
            }
            PlaybackCommand::Pause => {
                if let Some(id) = self.current_sink {
                    if let Some(sink) = self.sinks.get(&id) {
                        sink.pause();
                    }
                }
            }
            PlaybackCommand::Stop => {
                if let Some(id) = self.current_sink.take() {
                    // Remove the current sink from tracking
                    if let Some(sink) = self.sinks.get(&id) {
                        sink.stop(); // Stop the current sink
                    }
                }
            }
            PlaybackCommand::Resume => {
                if let Some(id) = self.current_sink {
                    if let Some(sink) = self.sinks.get(&id) {
                        sink.play(); // Resume the current sink
                    }
                }
            }
        }
        Ok(())
    }

    pub async fn play_audio(&mut self, audio_data: Vec<u8>) -> Result<SinkId, Box<dyn Error>> {
        let (stream, stream_handle) = OutputStream::try_default()?;
        let sink = Sink::try_new(&stream_handle)?;
        let source = Decoder::new(Cursor::new(audio_data))?;

        sink.append(source);

        // Assume playback starts immediately without blocking
        let id = self.next_id;
        self.sinks.insert(id, sink);
        self.streams.insert(id, stream);
        self.current_sink = Some(id); // Set current sink ID here
        self.next_id += 1;
        Ok(id)
    }
}

// endregion: --- Playback Manager
