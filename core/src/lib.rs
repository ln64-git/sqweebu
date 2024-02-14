// lib.rs

// region: --- imports
pub mod _utils;
use _utils::azure;
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
use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;
// endregion: --- imports

#[derive(Debug)]
pub struct AppState {
    pub running: Option<mpsc::Sender<()>>,
    pub playback_send: Sender<PlaybackCommand>,
}

impl Clone for AppState {
    fn clone(&self) -> Self {
        AppState {
            running: self.running.as_ref().map(|sender| sender.clone()),
            playback_send: self.playback_send.clone(),
        }
    }
}

type SinkId = usize;

#[derive(Debug, Clone)]
pub enum PlaybackCommand {
    Play(Vec<u8>),
    Pause,
    Stop,
    Resume,
}

pub struct PlaybackManager {
    pub next_id: SinkId,
    pub sinks: HashMap<SinkId, Sink>,
    pub streams: HashMap<SinkId, OutputStream>,
    pub command_queue: VecDeque<PlaybackCommand>,
    pub is_idle: AtomicBool,
    pub current_sink: Option<SinkId>,
}

impl PlaybackManager {
    pub fn new() -> Self {
        PlaybackManager {
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
                println!("HANDLE_COMMAND - Playing audio");
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

                println!("HANDLE_COMMAND - Audio playing on sink ID: {}", id);
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
}
