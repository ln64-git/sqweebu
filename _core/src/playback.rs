// src/_utils/playback.rs

// region: --- importswWE
use core::sync::atomic::AtomicBool;
use rodio::Decoder;
use rodio::{OutputStream, Sink};
use serde::Deserialize;
use serde::Serialize;
use std::collections::VecDeque;
use std::error::Error;
use std::io::Cursor;
use tokio::sync::mpsc::{self, Sender};
// endregion: --- imports

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum PlaybackCommand {
    Play(Vec<u8>),
    Pause,
    Stop,
    Resume,
    Clear,
}

pub struct PlaybackManager {
    pub command_queue: VecDeque<PlaybackCommand>,
    pub is_idle: AtomicBool,
    pub sink: Option<Sink>,
}

impl PlaybackManager {
    pub fn new(sink: Sink) -> Self {
        PlaybackManager {
            command_queue: VecDeque::new(),
            is_idle: AtomicBool::new(true),
            sink: Some(sink),
        }
    }

    pub async fn process_command_queue(&mut self) {
        while let Some(command) = self.command_queue.pop_front() {
            self.handle_command(command)
                .await
                .expect("Failed to handle command");
        }
    }

    pub async fn handle_command(&mut self, command: PlaybackCommand) -> Result<(), Box<dyn Error>> {
        match command {
            PlaybackCommand::Play(audio_data) => {
                if let Some(ref mut sink) = self.sink {
                    let source = Decoder::new(Cursor::new(audio_data))?;
                    sink.append(source);
                }
            }
            PlaybackCommand::Pause => {
                if let Some(ref mut sink) = self.sink {
                    sink.pause();
                }
            }
            PlaybackCommand::Stop => {
                if let Some(sink) = self.sink.take() {
                    sink.stop();
                }
            }
            PlaybackCommand::Resume => {
                if let Some(ref mut sink) = self.sink {
                    sink.play();
                }
            }
            PlaybackCommand::Clear => {
                if let Some(ref mut sink) = self.sink {
                    sink.clear();
                }
            }
        }
        Ok(())
    }
}

pub async fn init_playback_channel() -> Sender<PlaybackCommand> {
    let (playback_send, playback_recv) = mpsc::channel::<PlaybackCommand>(32);
    let (queue_send, queue_recv) = mpsc::channel::<PlaybackCommand>(32);

    tokio::spawn(command_queue_processor(playback_recv, queue_send.clone()));

    playback_execution_thread(queue_recv);

    playback_send
}

async fn command_queue_processor(
    mut playback_recv: mpsc::Receiver<PlaybackCommand>,
    queue_send: mpsc::Sender<PlaybackCommand>,
) {
    while let Some(command) = playback_recv.recv().await {
        let _ = queue_send.send(command).await;
    }
}

fn playback_execution_thread(mut queue_recv: mpsc::Receiver<PlaybackCommand>) {
    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let atomic_order = std::sync::atomic::Ordering::SeqCst;
        rt.block_on(async {
            let (_stream, stream_handle) = OutputStream::try_default().unwrap();
            let sink = Sink::try_new(&stream_handle).unwrap();

            let mut playback = PlaybackManager::new(sink);

            while let Some(command) = queue_recv.recv().await {
                playback.command_queue.push_back(command);
                if playback.is_idle.load(atomic_order) {
                    playback.is_idle.store(false, atomic_order);
                    playback.process_command_queue().await;
                    playback.is_idle.store(true, atomic_order);
                }
            }
        });
    });
}
