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
    pub running: Option<mpsc::Sender<()>>,
    pub playback_send: Sender<PlaybackCommand>,
    pub sentence_map: HashMap<usize, String>,
}

impl Clone for AppState {
    fn clone(&self) -> Self {
        AppState {
            running: self.running.as_ref().map(|sender| sender.clone()),
            playback_send: self.playback_send.clone(),
            sentence_map: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum PlaybackCommand {
    QueuePlayback(Arc<Mutex<AppState>>),
    Pause,
    Stop,
    Resume,
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
            PlaybackCommand::QueuePlayback(nexus) => {
                let state = nexus.lock().await;

                // Check if there are sentences in the queue
                if !state.sentence_map.is_empty() {
                    // Create a new sink for each playback
                    let (stream, stream_handle) = OutputStream::try_default()?;
                    let sink = Sink::try_new(&stream_handle)?;

                    // Iterate over references to sentences
                    for (_, sentence) in &state.sentence_map {
                        let audio_data = speak_text(sentence).await?;
                        let source = Decoder::new(Cursor::new(audio_data))?;
                        sink.append(source);
                    }

                    // Block until the audio playback is finished
                    while !sink.empty() {
                        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                    }

                    // Store the current sink
                    self.current_sink = Some(sink);
                }
            }

            PlaybackCommand::Pause => {
                println!("Pausing audio playback");
                if let Some(sink) = &mut self.current_sink {
                    sink.pause(); // Pause the current sink
                }
            }
            PlaybackCommand::Stop => {
                if let Some(sink) = self.current_sink.take() {
                    sink.stop(); // Stop the current sink
                }
            }
            PlaybackCommand::Resume => {
                if let Some(sink) = &self.current_sink {
                    sink.play(); // Resume the current sink
                }
            }
        }
        Ok(())
    }
}
