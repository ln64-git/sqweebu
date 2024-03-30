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
use tokio::time::{self, Duration}; // endregion: --- imports

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum PlaybackCommand {
    Play(AudioEntry),
    Pause(String), // Pause now carries a String argument.
    Stop,
    Resume,
    Clear,
    CheckSink,
}

pub struct PlaybackManager {
    pub command_queue: VecDeque<PlaybackCommand>,
    pub sink_empty: AtomicBool,
    pub sink: Option<Sink>,
    pub sentence_send: mpsc::Sender<String>,
    pub current_sentence: String,
    pub sentence_storage_send: mpsc::Sender<String>,
    pub sentence_storage_recv: mpsc::Receiver<String>,
    pub is_paused: AtomicBool,
}

impl PlaybackManager {
    pub fn new(sink: Sink, sentence_send: mpsc::Sender<String>) -> Self {
        let (sentence_storage_send, sentence_storage_recv) = mpsc::channel::<String>(32);
        PlaybackManager {
            command_queue: VecDeque::new(),
            sink_empty: AtomicBool::new(true),
            sink: Some(sink),
            sentence_send,
            sentence_storage_send,
            sentence_storage_recv,
            current_sentence: "".to_string(),
            is_paused: AtomicBool::new(false),
        }
    }

    pub async fn process_command_queue(&mut self) {
        while let Some(command) = self.command_queue.pop_front() {
            match &command {
                PlaybackCommand::Play(entry) => {
                    if self.is_paused.load(Ordering::SeqCst) {
                        // System is paused, re-queue the command to try later.
                        self.command_queue.push_back(command); // Re-queue the command itself
                        return; // Skip this iteration.
                    }
                    self.current_sentence = entry.text_content.clone();
                    let _ = self.sentence_send.send(entry.clone().text_content).await;
                    if self.sink_empty.load(Ordering::SeqCst) {
                        self.handle_play(entry.clone())
                            .await
                            .expect("Failed to handle play command");
                    }
                }
                PlaybackCommand::Pause(sentence) => {
                    println!("Processing Pause Command");
                    self.is_paused.store(true, Ordering::SeqCst);

                    // Send the current sentence to storage before pausing
                    let _ = self.sentence_storage_send.send(sentence.clone()).await;
                    // Clear the currently displayed sentence by sending an empty string
                    let _ = self.sentence_send.send("".to_string()).await;

                    // Clear the current_sentence since playback is paused
                    self.current_sentence = "".to_string();
                    println!("Current Sentence -->{:?}<--", self.current_sentence);

                    // Pause the audio playback
                    if let Some(ref sink) = self.sink {
                        sink.pause();
                    }
                }
                PlaybackCommand::Resume => {
                    println!("Processing Resume Command");
                    self.is_paused.store(false, Ordering::SeqCst);

                    // Attempt to retrieve the paused sentence from storage
                    let sentence_storage_result =
                        time::timeout(Duration::from_secs(5), self.sentence_storage_recv.recv())
                            .await;
                    match sentence_storage_result {
                        Ok(Some(sentence)) => {
                            // If a sentence is retrieved, send it to be displayed and resume playback
                            println!("Resuming with sentence: {:?}", sentence);
                            if let Some(ref mut sink) = self.sink {
                                let _ = self.sentence_send.send(sentence).await;
                                sink.play();
                            }
                        }
                        Ok(None) => {
                            println!(
                                "No more sentences to resume playback with, channel was closed."
                            );
                        }
                        Err(_) => {
                            println!("Timeout occurred waiting for sentence_storage_recv");
                        }
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
                    // If the sink is empty, send an empty string to indicate the current sentence should be cleared
                    if self.sink_empty.load(Ordering::SeqCst) {
                        let _ = self.sentence_send.send("".to_string()).await;
                    }
                }
            }
        }
    }

    async fn handle_play(&mut self, entry: AudioEntry) -> Result<(), Box<dyn Error>> {
        use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;
        if let Some(ref mut sink) = self.sink {
            let audio_data = BASE64_STANDARD
                .decode(entry.audio_data.as_bytes())
                .map_err(|e| Box::new(e) as Box<dyn Error>)?;
            let source = Decoder::new(Cursor::new(audio_data))?;
            sink.append(source);
            self.sink_empty.store(false, Ordering::SeqCst);
            while !sink.empty() {
                time::sleep(Duration::from_millis(100)).await;
            }
            self.sink_empty.store(true, Ordering::SeqCst);
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
