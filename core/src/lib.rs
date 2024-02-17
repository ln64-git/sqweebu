// lib.rs

// region: --- imports
pub mod _utils;
use _utils::azure;
use _utils::azure::speak_text;
use _utils::ollama;
use _utils::playback;
use rodio::Decoder;
use rodio::OutputStream;
use rodio::Sink;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::error::Error;
use std::io::Cursor;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;
use tokio::sync::Mutex;
// endregion: --- imports

#[derive(Debug)]
pub struct AppState {
    pub playback_send: Sender<PlaybackCommand>,
    pub sentence_map: Arc<Mutex<HashMap<usize, String>>>, // Wrap HashMap in Arc<Mutex<>>
}

impl Clone for AppState {
    fn clone(&self) -> Self {
        AppState {
            playback_send: self.playback_send.clone(),
            sentence_map: Arc::clone(&self.sentence_map), // Clone the Arc
        }
    }
}

#[derive(Debug, Clone)]
pub enum PlaybackCommand {
    Play(Vec<u8>),
    Pause,
    Stop,
    Resume,
    GetLength,
}

pub struct PlaybackManager {
    pub command_queue: VecDeque<PlaybackCommand>,
    pub is_idle: AtomicBool,
    pub current_sink: Option<Sink>,
}

impl PlaybackManager {
    pub fn new() -> Self {
        PlaybackManager {
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
                let (stream, stream_handle) = OutputStream::try_default()?;
                let sink = Sink::try_new(&stream_handle)?;
                let source = Decoder::new(Cursor::new(audio_data))?;
                sink.append(source);
                sink.sleep_until_end();
                self.current_sink = Some(sink);
            }

            PlaybackCommand::Pause => {
                println!("Pausing audio playback");
                if let Some(ref mut sink) = self.current_sink {
                    sink.pause(); // Pause the current sink
                }
            }
            PlaybackCommand::Stop => {
                if let Some(sink) = self.current_sink.take() {
                    sink.stop(); // Stop the current sink
                }
            }
            PlaybackCommand::Resume => {
                if let Some(ref mut sink) = self.current_sink {
                    sink.play(); // Resume the current sink
                }
            }
            PlaybackCommand::GetLength => {
                if let Some(ref mut sink) = self.current_sink {
                    sink.len(); // Return the current sink
                }
            }
        }
        Ok(())
    }
}
