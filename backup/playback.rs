use crate::{speak_ollama, SpeechState};
use actix_web::{web, Responder};
use std::sync::atomic::Ordering;
use std::sync::{Arc, Mutex};

pub async fn pause(state: web::Data<Arc<Mutex<SpeechState>>>) -> impl Responder {
    let state = state.lock().unwrap();
    if let Some(sink) = state.current_sink.as_ref() {
        let mut sink = sink.lock().unwrap();
        sink.pause();
        state.is_paused.store(true, Ordering::SeqCst);
        "Playback paused"
    } else {
        "No active playback to pause"
    }
}

pub async fn resume(state: web::Data<Arc<Mutex<SpeechState>>>) -> impl Responder {
    let state = state.lock().unwrap();
    if let Some(sink) = state.current_sink.as_ref() {
        let mut sink = sink.lock().unwrap();
        if state.is_paused.load(Ordering::SeqCst) {
            sink.play();
            state.is_paused.store(false, Ordering::SeqCst);
            "Playback resumed"
        } else {
            "Playback is not paused"
        }
    } else {
        "No active playback to resume"
    }
}
pub async fn play(
    data: web::Data<Arc<Mutex<SpeechState>>>, // The type of `data` is web::Data wrapping Arc<Mutex<SpeechState>>
    web::Json(prompt): web::Json<String>,
) -> impl Responder {
    // Correctly extract Arc<Mutex<SpeechState>> from web::Data without additional Arc wrapping
    let state_arc = data.into_inner();
    // Now pass `state_arc` directly to `speak_ollama`
    tokio::spawn(async move {
        if let Err(e) = speak_ollama(prompt).await {
            // Ensure this matches the expected function signature
            eprintln!("Error during playback: {}", e);
        }
    });

    "Playback started"
}
// pub async fn stop(
//     data: web::Data<Arc<Mutex<SpeechState>>>, // The type of `data` is web::Data wrapping Arc<Mutex<SpeechState>>
//     web::Json(prompt): web::Json<String>,
// ) -> impl Responder {
//     // TODO
//     "Playback stopped"
// }
