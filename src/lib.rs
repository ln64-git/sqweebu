// src/lib.rs

mod _api;
mod _utils;

// region: --- crates
pub use crate::_api::azure::azure_response_to_audio;
pub use crate::_api::azure::get_azure_response;
pub use crate::_api::ollama::ollama_generate_api;
pub use crate::_api::ollama::speak_ollama;
pub use crate::_utils::audio::play_audio_data;
pub use crate::_utils::audio::speak_text;
pub use crate::_utils::clipboard::get_clipboard;
pub use crate::_utils::clipboard::speak_clipboard;
// endregion: --- crates

// region: --- imports
use rodio::Sink;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};
// endregion: --- imports

pub struct SpeechState {
    pub current_sink: Option<Arc<Mutex<Sink>>>,
    pub is_paused: AtomicBool,
    // Add a new field to store the prompt or other playback relevant data
    pub prompt: Arc<Mutex<Option<String>>>,
}

impl SpeechState {
    pub fn new() -> Self {
        SpeechState {
            current_sink: None,
            is_paused: AtomicBool::new(false),
            prompt: Arc::new(Mutex::new(None)),
        }
    }
    pub fn set_prompt(&mut self, prompt: String) {
        let mut prompt_lock = self.prompt.lock().unwrap();
        *prompt_lock = Some(prompt);
    }

    pub fn clear_prompt(&mut self) {
        let mut prompt_lock = self.prompt.lock().unwrap();
        *prompt_lock = None;
    }
}
