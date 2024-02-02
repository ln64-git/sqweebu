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
use actix_web::{web, App, HttpServer};
use rodio::Decoder;
use rodio::OutputStream;
use rodio::OutputStreamHandle;
use rodio::Sink;
use serde::Deserialize;
use serde::Serialize;
use std::error::Error;
use std::io::Cursor;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};
// endregion: --- imports

pub struct AudioPlaybackManager {
    sink: Option<Sink>,
    stream_handle: OutputStreamHandle,
}

impl AudioPlaybackManager {
    pub fn new() -> Self {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        AudioPlaybackManager {
            sink: None,
            stream_handle,
        }
    }

    pub fn play_audio(&mut self, audio_data: Vec<u8>) -> Result<(), Box<dyn Error>> {
        let sink = Sink::try_new(&self.stream_handle)?;
        let source = Decoder::new(Cursor::new(audio_data))?;
        sink.append(source);
        self.sink = Some(sink);
        Ok(())
    }

    pub fn pause_audio(&mut self, audio_data: Vec<u8>) -> Result<(), Box<dyn Error>> {
        // Pause using rodio sink
        Ok(())
    }

    pub fn resume_audio(&mut self, audio_data: Vec<u8>) -> Result<(), Box<dyn Error>> {
        // Resume using rodio sink
        Ok(())
    }

    pub fn stop_audio(&mut self, audio_data: Vec<u8>) -> Result<(), Box<dyn Error>> {
        // Stop using rodio sink
        Ok(())
    }
}


