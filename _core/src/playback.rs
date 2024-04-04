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
use std::sync::Arc;
use tokio::sync::mpsc::{self, Sender};
use tokio::time::Duration;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum PlaybackCommand {
    Play(AudioEntry),
    Pause,
    Stop,
    Resume,
    Clear,
    CheckSink,
}

pub struct PlaybackManager {
    pub sink: Option<Sink>,
    pub sink_empty: Arc<AtomicBool>,
    pub is_paused: AtomicBool,
    pub command_queue: VecDeque<PlaybackCommand>,
    pub current_entry: Option<AudioEntry>,
    pub entry_send: mpsc::Sender<Option<AudioEntry>>,
}

impl PlaybackManager {
    pub fn new(sink: Sink, entry_send: mpsc::Sender<Option<AudioEntry>>) -> Self {
        PlaybackManager {
            sink: Some(sink),
            sink_empty: Arc::new(AtomicBool::new(true)),
            is_paused: AtomicBool::new(false),
            command_queue: VecDeque::new(),
            current_entry: None,
            entry_send,
        }
    }

    pub async fn process_command_queue(&mut self) {
        while let Some(command) = self.command_queue.pop_front() {
            match command {
                PlaybackCommand::Play(ref entry) => {
                    if self.is_paused.load(Ordering::SeqCst) {
                        self.command_queue.push_back(command); // Re-queue the command itself
                        return;
                    }
                    self.current_entry = Some(entry.clone());
                    let _ = self.entry_send.send(Some(entry.clone())).await;
                    self.handle_play(entry.clone())
                        .await
                        .expect("Failed to handle play command");
                }
                PlaybackCommand::Pause => {
                    self.is_paused.store(true, Ordering::SeqCst);
                    let _ = self.entry_send.send(None).await;
                    self.current_entry = None;
                    if let Some(ref sink) = self.sink {
                        sink.pause();
                    }
                }
                PlaybackCommand::Resume => {
                    if self.is_paused.load(Ordering::SeqCst) {
                        self.is_paused.store(false, Ordering::SeqCst);
                        if let Some(ref sink) = self.sink {
                            sink.play();
                        }
                    }
                }
                PlaybackCommand::Stop => {
                    if let Some(ref mut sink) = self.sink.take() {
                        sink.stop();
                        self.current_entry = None;
                    }
                }
                PlaybackCommand::Clear => {
                    if let Some(ref mut sink) = self.sink {
                        sink.clear();
                    }
                }
                PlaybackCommand::CheckSink => {
                    if self.sink_empty.load(Ordering::SeqCst) {
                        let _ = self.entry_send.send(None).await;
                    }
                }
            }
        }
    }

    async fn handle_play(&mut self, entry: AudioEntry) -> Result<(), Box<dyn Error>> {
        use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;

        if entry.text_content.trim().is_empty() {
            return Ok(());
        }

        if let Some(ref mut sink) = self.sink {
            let audio_data = BASE64_STANDARD
                .decode(entry.audio_data.as_bytes())
                .map_err(|e| {
                    eprintln!("Error decoding base64 audio data: {}", e);
                    Box::new(e) as Box<dyn Error>
                })?;

            match Decoder::new(Cursor::new(audio_data)) {
                Ok(source) => {
                    // Immediately set sink_empty to false since we're starting playback
                    self.sink_empty.store(false, Ordering::SeqCst);

                    // Append the audio source to the sink for playback
                    sink.append(source);

                    // Calculate the delay duration based on audio_length
                    let audio_length = Duration::from_secs_f32(entry.audio_length);
                    let text = entry.text_content;

                    // Clone the sink_empty Arc to move into the async block
                    let sink_empty_clone = self.sink_empty.clone();

                    // Start an asynchronous delay based on the audio_length
                    tokio::spawn(async move {
                        println!("{:#?}", text);
                        tokio::time::sleep(audio_length).await;
                        println!("{:#?}", audio_length);
                        // Once the delay is over, mark the sink as empty
                        sink_empty_clone.store(true, Ordering::SeqCst);
                    });
                }
                Err(e) => {
                    eprintln!("Error creating audio source decoder: {}", e);
                }
            }
        }

        Ok(())
    }
}

pub async fn init_playback_channel(
    entry_send: mpsc::Sender<Option<AudioEntry>>,
) -> Sender<PlaybackCommand> {
    let (playback_send, mut playback_recv) = mpsc::channel::<PlaybackCommand>(32);
    let (queue_send, mut queue_recv) = mpsc::channel::<PlaybackCommand>(32);

    tokio::spawn(async move {
        while let Some(command) = playback_recv.recv().await {
            let _ = queue_send.send(command).await;
        }
    });

    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        let mut playback = PlaybackManager::new(sink, entry_send);

        rt.block_on(async {
            while let Some(command) = queue_recv.recv().await {
                playback.command_queue.push_back(command);
                playback.process_command_queue().await;
            }
        });
    });

    playback_send
}
