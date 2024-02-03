// src/lib.rs

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
pub use crate::_utils::endpoints::speak_clipboard_endpoint;
pub use crate::_utils::endpoints::speak_ollama_endpoint;
// endregion: --- crates

// region: --- imports
use rodio::Decoder;
use rodio::OutputStream;
use rodio::Sink;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::error::Error;
use std::io::Cursor;
use std::sync::atomic::AtomicBool;
use tokio::sync::mpsc;
// endregion: --- imports

pub struct AppState {
    pub tx: mpsc::Sender<PlaybackCommand>,
}

// Keeping SinkId as the mechanism for referencing sinks
type SinkId = usize;

pub enum PlaybackCommand {
    Play(Vec<u8>), // Play audio data
}

pub struct AudioPlaybackManager {
    pub next_id: SinkId,
    pub sinks: HashMap<SinkId, Sink>,
    pub streams: HashMap<SinkId, OutputStream>,
    pub command_queue: VecDeque<PlaybackCommand>,
    pub is_idle: AtomicBool,
}

impl AudioPlaybackManager {
    pub fn new() -> Self {
        AudioPlaybackManager {
            next_id: 0,
            sinks: HashMap::new(),
            streams: HashMap::new(),
            command_queue: VecDeque::new(),
            is_idle: AtomicBool::new(true),
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
        }
        Ok(())
    }

    pub async fn play_audio(&mut self, audio_data: Vec<u8>) -> Result<SinkId, Box<dyn Error>> {
        // Attempt to create an OutputStream and a Sink for playing audio
        let (stream, stream_handle) = OutputStream::try_default()?;
        let sink = Sink::try_new(&stream_handle)?;
        let source = Decoder::new(Cursor::new(audio_data))?;
        sink.append(source);
        while !sink.empty() {
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }

        // Assign an ID to this audio stream for management
        let id = self.next_id;
        self.sinks.insert(id, sink);
        self.streams.insert(id, stream); // Keep the OutputStream alive
        self.next_id += 1;
        Ok(id)
    }

    pub fn pause_audio(&mut self, id: SinkId) {
        if let Some(sink) = self.sinks.get_mut(&id) {
            sink.pause();
        }
    }

    pub fn resume_audio(&mut self, id: SinkId) {
        if let Some(sink) = self.sinks.get_mut(&id) {
            sink.play();
        }
    }

    pub fn stop_audio(&mut self, id: SinkId) {
        if let Some(sink) = self.sinks.remove(&id) {
            sink.stop();
        }
        self.streams.remove(&id);
    }
}
