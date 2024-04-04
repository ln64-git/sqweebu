// src/_utils/playback.rs

// region: --- importswWE
use crate::utils::{check_empty_sink, AudioEntry};
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
use tokio::time::{self, Duration};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum PlaybackCommand {
    Play(AudioEntry),
    Pause(String),
    Stop,
    Resume,
    Clear,
    CheckSink,
}

pub struct PlaybackManager {
    pub sink: Option<Sink>,
    pub sink_empty: Arc<AtomicBool>,
    pub is_paused: Arc<AtomicBool>,
    pub command_queue: VecDeque<PlaybackCommand>,
    pub current_sentence: String,
    pub playback_send: mpsc::Sender<PlaybackCommand>,
    pub sentence_send: mpsc::Sender<String>,
    pub sentence_storage_send: mpsc::Sender<String>,
    pub sentence_storage_recv: mpsc::Receiver<String>,
}

impl PlaybackManager {
    pub fn new(
        sink: Sink,
        sentence_send: mpsc::Sender<String>,
        playback_send: mpsc::Sender<PlaybackCommand>,
    ) -> Self {
        let (sentence_storage_send, sentence_storage_recv) = mpsc::channel::<String>(32);
        PlaybackManager {
            sink: Some(sink),
            sink_empty: Arc::new(AtomicBool::new(true)),
            is_paused: Arc::new(AtomicBool::new(false)),
            command_queue: VecDeque::new(),
            current_sentence: "".to_string(),
            playback_send,
            sentence_send,
            sentence_storage_send,
            sentence_storage_recv,
        }
    }

    pub async fn process_command_queue(&mut self) {
        while let Some(command) = self.command_queue.pop_front() {
            match &command {
                PlaybackCommand::Play(entry) => {
                    if self.is_paused.load(Ordering::SeqCst) {
                        self.command_queue.push_back(command); // Re-queue the command itself
                        return;
                    }
                    self.current_sentence = entry.text_content.clone();
                    let _ = self.sentence_send.send(entry.clone().text_content).await;
                    self.handle_play(entry.clone())
                        .await
                        .expect("Failed to handle play command");
                }
                PlaybackCommand::Pause(sentence) => {
                    self.is_paused.store(true, Ordering::SeqCst);
                    let _ = self.sentence_storage_send.send(sentence.clone()).await;
                    let _ = self.sentence_send.send("".to_string()).await;
                    self.current_sentence = "".to_string();
                    if let Some(ref sink) = self.sink {
                        sink.pause();
                    }
                }
                PlaybackCommand::Resume => {
                    self.is_paused.store(false, Ordering::SeqCst);
                    match time::timeout(Duration::from_secs(5), self.sentence_storage_recv.recv())
                        .await
                    {
                        Ok(Some(sentence)) => {
                            if let Some(sink) = &mut self.sink {
                                let _ = self.sentence_send.send(sentence).await;
                                sink.play();
                            }
                        }
                        Ok(None) => println!(
                            "No more sentences to resume playback with, channel was closed."
                        ),
                        Err(_) => println!("Timeout occurred waiting for sentence_storage_recv"),
                    }
                }

                PlaybackCommand::Stop => {
                    if let Some(ref mut sink) = self.sink.take() {
                        sink.stop();
                        self.current_sentence = "".to_string();
                    }
                }
                PlaybackCommand::Clear => {
                    if let Some(ref mut sink) = self.sink {
                        sink.clear();
                    }
                }
                PlaybackCommand::CheckSink => {
                    if self.sink_empty.load(Ordering::SeqCst) {
                        let _ = self.sentence_send.send("".to_string()).await;
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
                    sink.append(source);
                    self.sink_empty.store(false, Ordering::SeqCst);
                    // sink.sleep_until_end();
                    let playback_send_clone = self.playback_send.clone();
                    let sink_empty_clone = Arc::clone(&self.sink_empty);
                    let is_paused_clone = Arc::clone(&self.is_paused);
                    tokio::spawn(async move {
                        while !sink_empty_clone.load(Ordering::SeqCst)
                            && !is_paused_clone.load(Ordering::SeqCst)
                        {
                            time::sleep(Duration::from_millis(100)).await;
                            let sink_empty = check_empty_sink(&playback_send_clone)
                                .await
                                .unwrap_or(false);
                            if sink_empty {
                                sink_empty_clone.store(true, Ordering::SeqCst);
                            }
                        }
                    });
                    self.sink_empty.store(true, Ordering::SeqCst);
                }
                Err(e) => {
                    eprintln!("Error creating audio source decoder: {}", e);
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

    let playback_send_clone_for_thread = playback_send.clone(); // Clone for use in the thread
    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        let mut playback =
            PlaybackManager::new(sink, sentence_send, playback_send_clone_for_thread);

        rt.block_on(async {
            while let Some(command) = queue_recv.recv().await {
                playback.command_queue.push_back(command);
                playback.process_command_queue().await;
            }
        });
    });

    playback_send // This is now fine since we've cloned before moving
}
