// src/_utils/playback.rs

// region: --- importswWE
use crate::utils::AudioEntry;
use base64::Engine;
use core::sync::atomic::AtomicBool;
use rodio::Decoder;
use rodio::{OutputStream, Sink};
use serde::Deserialize;
use serde::Serialize;
use std::collections::VecDeque;
use std::error::Error;
use std::io::Cursor;
use std::sync::atomic::Ordering;
use tokio::sync::mpsc::{self, Sender};
// endregion: --- imports

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum PlaybackCommand {
    Play(AudioEntry),
    Pause,
    Stop,
    Resume,
    Clear,
}

pub struct PlaybackManager {
    pub command_queue: VecDeque<PlaybackCommand>,
    pub sink_empty: AtomicBool,
    pub sink: Option<Sink>,
    pub sentence_send: mpsc::Sender<String>,
}

impl PlaybackManager {
    pub fn new(sink: Sink, sentence_send: mpsc::Sender<String>) -> Self {
        PlaybackManager {
            command_queue: VecDeque::new(),
            sink_empty: AtomicBool::new(true),
            sink: Some(sink),
            sentence_send,
        }
    }

    pub async fn process_command_queue(&mut self) {
        while let Some(command) = self.command_queue.pop_front() {
            match command {
                PlaybackCommand::Play(entry) => {
                    let _ = self.sentence_send.send(entry.clone().text_content).await;
                    if self.sink_empty.load(Ordering::SeqCst) {
                        self.handle_command(PlaybackCommand::Play(entry.clone()))
                            .await
                            .expect("Failed to handle command");
                    } else {
                        break;
                    }
                }
                _ => {
                    self.handle_command(command)
                        .await
                        .expect("Failed to handle command");
                }
            }
        }
    }

    pub async fn handle_command(&mut self, command: PlaybackCommand) -> Result<(), Box<dyn Error>> {
        use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;
        match command {
            PlaybackCommand::Play(entry) => {
                if let Some(ref mut sink) = self.sink {
                    let audio_data = BASE64_STANDARD
                        .decode(entry.audio_data.as_bytes())
                        .map_err(|e| Box::new(e) as Box<dyn Error>)?;

                    let source = Decoder::new(Cursor::new(audio_data))?;
                    sink.append(source);
                    self.sink_empty.store(false, Ordering::SeqCst); // Set the playing flag
                    sink.sleep_until_end();
                    self.sink_empty.store(true, Ordering::SeqCst); // Reset the playing flag                }
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

pub async fn init_playback_channel(sentence_send: mpsc::Sender<String>) -> Sender<PlaybackCommand> {
    let (playback_send, mut playback_recv) = mpsc::channel::<PlaybackCommand>(32);
    let (queue_send, mut queue_recv) = mpsc::channel::<PlaybackCommand>(32);

    tokio::spawn(async move {
        while let Some(command) = playback_recv.recv().await {
            let _ = queue_send.send(command).await;
        }
    });

    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let (_stream, stream_handle) = OutputStream::try_default().unwrap();
            let sink = Sink::try_new(&stream_handle).unwrap();

            let mut playback = PlaybackManager::new(sink, sentence_send);

            while let Some(command) = queue_recv.recv().await {
                playback.command_queue.push_back(command);
                playback.process_command_queue().await;
            }
        });
    });

    playback_send
}
