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
use std::sync::atomic::Ordering;
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
    QueuePlayback(Vec<u8>),
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
    pub current_sink: Option<SinkId>, // New field to track the current playing audio
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
            PlaybackCommand::QueuePlayback(audio_data) => {
                println!("Playing audio");
                self.play_audio(audio_data).await?;
            }
            PlaybackCommand::Pause => {
                println!("Pausing audio playback");
                if let Some(id) = self.current_sink {
                    if let Some(sink) = self.sinks.get(&id) {
                        println!("id: {}", id);
                        sink.pause(); // Pause the current sink
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
        // Attempt to create an OutputStream and a Sink for playing audio
        let (stream, stream_handle) = OutputStream::try_default()?;
        let sink = Sink::try_new(&stream_handle)?;
        let source = Decoder::new(Cursor::new(audio_data))?;
        sink.append(source);
        while !sink.empty() {
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
        let id = self.next_id;
        self.sinks.insert(id, sink);
        self.streams.insert(id, stream);
        self.next_id += 1;
        Ok(id)
    }
}
