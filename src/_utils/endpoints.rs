// src/utils/endpoints.rs

use actix_web::{web, HttpResponse, Responder};
use std::sync::Mutex;
use tokio::sync::mpsc;

use crate::{speak_clipboard, speak_ollama, AppState, PlaybackCommand};

pub async fn speak_clipboard_endpoint(data: web::Data<Mutex<AppState>>) -> impl Responder {
    let tx = {
        let lock = data.lock().unwrap();
        lock.tx.clone()
    };

    match speak_clipboard(tx).await {
        Ok(_) => HttpResponse::Ok().body("Clipboard content spoken."),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

// Ollama content to speech
pub async fn speak_ollama_endpoint(
    body: web::Json<String>,
    data: web::Data<Mutex<AppState>>,
) -> impl Responder {
    let preface = "In three sentences explain...";
    let final_prompt = format!("{} {}", preface, *body);

    let tx = {
        let lock = data.lock().unwrap();
        lock.tx.clone()
    };

    match speak_ollama(final_prompt, tx).await {
        Ok(_) => HttpResponse::Ok().body("Ollama content spoken."),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

// Pause audio playback
pub async fn pause_audio_endpoint(
    data: web::Data<Mutex<AppState>>,
    id: web::Path<usize>, // Correctly use web::Path
) -> impl Responder {
    let tx = data.lock().unwrap().tx.clone();
    if tx.send(PlaybackCommand::Pause(*id)).await.is_err() {
        // Dereference id with * since web::Path implements Deref
        return HttpResponse::InternalServerError().body("Failed to send pause command.");
    }
    HttpResponse::Ok().body(format!("Pause command sent for ID: {}", id))
}

pub async fn resume_audio_endpoint(
    data: web::Data<Mutex<AppState>>,
    id: web::Path<usize>, // Correctly use web::Path
) -> impl Responder {
    let tx = data.lock().unwrap().tx.clone();
    if tx.send(PlaybackCommand::Resume(*id)).await.is_err() {
        return HttpResponse::InternalServerError().body("Failed to send resume command.");
    }
    HttpResponse::Ok().body("Resume command sent.")
}

pub async fn stop_audio_endpoint(
    data: web::Data<Mutex<AppState>>,
    id: web::Path<usize>, // Correctly use web::Path
) -> impl Responder {
    let tx = data.lock().unwrap().tx.clone();
    if tx.send(PlaybackCommand::Stop(*id)).await.is_err() {
        return HttpResponse::InternalServerError().body("Failed to send stop command.");
    }
    HttpResponse::Ok().body("Stop command sent.")
}
