// lib.rs

// region: --- imports

pub mod _utils;
use _utils::playback;
use rodio::Decoder;
use rodio::OutputStream;
use rodio::Sink;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::collections::VecDeque;
use std::error::Error;
use std::io::Cursor;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;
use tokio::sync::Mutex;
// endregion: --- imports

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserSettings {
    pub gpt_method: String,
    pub gpt_model: String,
    pub speech_service: String,
    pub speech_local: String,
    pub speech_voice: String,
    pub current_user_id: String,
    pub current_user_theme: String,
    pub current_user_theme_mode: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User {
    pub user_id: String,
    pub user_name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserMessage {
    pub user_id: String,
    pub message_id: String,
    pub message_body: String,
}

pub struct AppState {
    pub playback_send: mpsc::Sender<PlaybackCommand>,
    pub user_settings: Option<UserSettings>,
    pub user_array: Vec<User>,
    pub user_messages_array: Vec<UserMessage>,
}

impl Default for UserSettings {
    fn default() -> Self {
        Self {
            gpt_method: String::default(),
            gpt_model: String::default(),
            speech_service: String::default(),
            speech_local: String::default(),
            speech_voice: String::default(),
            current_user_id: String::default(),
            current_user_theme: String::default(),
            current_user_theme_mode: String::default(),
        }
    }
}

impl Clone for AppState {
    fn clone(&self) -> Self {
        AppState {
            playback_send: self.playback_send.clone(),
            user_settings: self.user_settings.clone(),
            user_array: self.user_array.clone(),
            user_messages_array: self.user_messages_array.clone(),
        }
    }
}

impl AppState {
    pub fn set_user_settings(&mut self, user_settings: UserSettings) {
        self.user_settings = Some(user_settings);
    }

    pub fn get_user_settings(&self) -> Option<&UserSettings> {
        self.user_settings.as_ref()
    }

    pub fn add_user(&mut self, user: User) {
        self.user_array.push(user);
    }

    pub fn add_user_message(&mut self, user_message: UserMessage) {
        self.user_messages_array.push(user_message);
    }
}

// region: --- Playback Manager

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum PlaybackCommand {
    Play(Vec<u8>),
    Pause,
    Stop,
    Resume,
    FastForward,
    Rewind,
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
                println!("Pausing audio playback");
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
            PlaybackCommand::FastForward => {
                if let Some(ref mut sink) = self.sink {
                    sink.skip_one();
                }
            }
            PlaybackCommand::Rewind => {
                if let Some(ref mut sink) = self.sink {
                    // TODO
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

// endregion: --- Playback Manager
