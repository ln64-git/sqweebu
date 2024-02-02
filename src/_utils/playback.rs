// src/utils/playback.rs

// region: --- modules
use crate::AudioPlaybackManager;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use rodio::{Decoder, OutputStream, Sink};
use std::error::Error;
use std::io::Cursor;
use std::sync::{Arc, Mutex};
// endregion: --- modules

// Endpoint to play audio (for demo, this will need actual audio data)
pub async fn play_endpoint(data: web::Data<Mutex<AudioPlaybackManager>>) -> impl Responder {
    let mut manager = data.lock().unwrap();
    // Here you would replace this Vec<u8> with actual audio data
    // TODO: Replace this with actual audio data
    // user makes api calls to /speak_clipboard and /speak_ollama
    //
    let dummy_audio_data = Vec::new(); // Placeholder for audio data

    manager
        .play_audio(dummy_audio_data)
        .expect("Failed to play audio");
    HttpResponse::Ok().body("Audio playback started")
}

// Endpoint to pause audio
pub async fn pause_endpoint(data: web::Data<Arc<Mutex<AudioPlaybackManager>>>) -> impl Responder {
    let mut manager = data.lock().unwrap();
    manager.pause_audio();
    HttpResponse::Ok().body("Audio playback paused")
}

// Endpoint to resume audio
pub async fn resume_endpoint(data: web::Data<Arc<Mutex<AudioPlaybackManager>>>) -> impl Responder {
    let mut manager = data.lock().unwrap();
    manager.resume_audio();
    HttpResponse::Ok().body("Audio playback resumed")
}

// Endpoint to stop audio
pub async fn stop_endpoint(data: web::Data<Arc<Mutex<AudioPlaybackManager>>>) -> impl Responder {
    let mut manager = data.lock().unwrap();
    manager.stop_audio();
    HttpResponse::Ok().body("Audio playback stopped")
}
