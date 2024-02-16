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
    QueueSentenceForPlayback(Vec<u8>),
    Pause,
    Stop,
    Resume,
}

pub struct PlaybackManager {
    pub current_sink_id: Option<SinkId>,
    pub next_sink_id: SinkId,
    pub sinks_array: HashMap<SinkId, Sink>,
    pub stream_array: HashMap<SinkId, OutputStream>,
    pub command_queue: VecDeque<PlaybackCommand>,
    pub is_idle: AtomicBool,
    pub sink: Sink,
}

impl PlaybackManager {
    pub fn new() -> Self {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).ok();
        PlaybackManager {
            next_sink_id: 0,
            sinks_array: HashMap::new(),
            stream_array: HashMap::new(),
            command_queue: VecDeque::new(),
            is_idle: AtomicBool::new(true),
            current_sink_id: None,
            sink: sink.unwrap(),
        }
    }

    pub async fn start_processing_commands(&mut self) {
        while let Some(command) = self.command_queue.pop_front() {
            self.handle_command(command)
                .await
                .expect("Failed to handle command");
        }
    }

    pub async fn playback_queue(&mut self) {
        while let Some(command) = self.command_queue.pop_front() {
            println!("command: {:#?}", command);

            self.handle_command(command)
                .await
                .expect("Failed to handle command");
        }
    }

    async fn handle_command(&mut self, command: PlaybackCommand) -> Result<(), Box<dyn Error>> {
        match command {
            PlaybackCommand::QueueSentenceForPlayback(audio_data) => {
                println!("HANDLE_COMMAND - Playing audio");
                println!("self.sink: {:#?}", self.current_sink_id);
                let source = Decoder::new(Cursor::new(audio_data))?;
                self.sink.append(source);
                self.is_idle.store(false, Ordering::Relaxed); // Set idle to false
                self.sink.play(); // Start playback
                println!("HANDLE_COMMAND - Audio playing");
            }
            PlaybackCommand::Pause => {
                self.sink.pause();
            }
            PlaybackCommand::Stop => {
                self.sink.stop();
                self.is_idle.store(true, Ordering::Relaxed); // Set idle to true
            }
            PlaybackCommand::Resume => {
                self.sink.play();
            }
        }
        Ok(())
    }

    fn get_sink_id(&mut self) -> Option<SinkId> {
        self.current_sink_id
    }
}
