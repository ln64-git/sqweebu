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
use rodio::cpal::traits::StreamTrait;
use rodio::Decoder;
use rodio::OutputStream;
use rodio::OutputStreamHandle;
use rodio::Sink;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::error::Error;
use std::io::Cursor;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};
// endregion: --- imports

type SinkId = usize;

pub struct AudioPlaybackManager {
    sinks: HashMap<SinkId, Sink>,
    streams: HashMap<SinkId, OutputStream>, // Store OutputStreams to ensure they live as needed
    next_id: SinkId,
}

impl AudioPlaybackManager {
    pub fn new() -> Self {
        AudioPlaybackManager {
            sinks: HashMap::new(),
            streams: HashMap::new(), // Initialize the HashMap for OutputStream objects
            next_id: 0,
        }
    }
    pub fn play_audio(&mut self, audio_data: Vec<u8>) -> Result<SinkId, Box<dyn Error>> {
        let (stream, stream_handle) = OutputStream::try_default()?;
        let sink = Sink::try_new(&stream_handle)?;
        let source = Decoder::new(Cursor::new(audio_data))?;
        sink.append(source);

        let id = self.next_id;
        self.sinks.insert(id, sink);
        self.streams.insert(id, stream); // Keep the OutputStream alive
        self.next_id += 1;

        Ok(id)
    }

    pub fn stop_audio(&mut self, id: SinkId) {
        if let Some(sink) = self.sinks.remove(&id) {
            sink.stop();
        }
    }

    pub fn pause_audio(&mut self, id: SinkId) {
        if let Some(sink) = self.sinks.get_mut(&id) {
            sink.pause();
        }
    }

    pub fn resume_audio(&mut self, id: SinkId) {
        if let Some(sink) = self.sinks.get_mut(&id) {
            sink.play();
        }
    }
}
