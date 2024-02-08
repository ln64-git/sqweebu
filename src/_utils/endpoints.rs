// src/_utils/endpoints.rs

use actix_web::{web, HttpResponse, Responder};
use std::sync::Mutex;

use crate::{speak_clipboard, speak_ollama, AppState, PlaybackCommand};

// region: --- Main Endpoints
pub async fn speak_clipboard_endpoint(data: web::Data<Mutex<AppState>>) -> impl Responder {
    let control_tx = {
        let lock = data.lock().unwrap();
        lock.control_tx.clone()
    };

    match speak_clipboard(control_tx).await {
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

    let control_tx = {
        let lock = data.lock().unwrap();
        lock.control_tx.clone()
    };

    match speak_ollama(final_prompt, control_tx).await {
        Ok(_) => HttpResponse::Ok().body("Ollama content spoken."),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}
// endregion: --- Main Endpoints

// region: --- Recording Endpoints

pub async fn record_start_endpoint(data: web::Data<Mutex<AppState>>) -> impl Responder {
    let control_tx = {
        let lock = data.lock().unwrap();
        lock.control_tx.clone()
    };

    if let Err(e) = control_tx.send(PlaybackCommand::Pause).await {
        println!("Error sending start command: {}", e);
        return HttpResponse::InternalServerError()
            .body(format!("Error sending start command: {}", e));
    }
    HttpResponse::Ok().body("Recording started.")
}

pub async fn record_stop_endpoint(data: web::Data<Mutex<AppState>>) -> impl Responder {
    let control_tx = {
        let lock = data.lock().unwrap();
        lock.control_tx.clone()
    };

    if let Err(e) = control_tx.send(PlaybackCommand::Pause).await {
        println!("Error sending stop command: {}", e);
        return HttpResponse::InternalServerError()
            .body(format!("Error sending stop command: {}", e));
    }
    HttpResponse::Ok().body("Recording Ended.")
}

// endregion: --- Recording Endpoints

// region: --- Playback Endpoints

// Stop playback
pub async fn playback_pause_endpoint(data: web::Data<Mutex<AppState>>) -> impl Responder {
    let control_tx = {
        let lock = data.lock().unwrap();
        lock.control_tx.clone()
    };

    if let Err(e) = control_tx.send(PlaybackCommand::Pause).await {
        println!("Error sending pause command: {}", e);
        return HttpResponse::InternalServerError()
            .body(format!("Error sending pause command: {}", e));
    }
    HttpResponse::Ok().body("Playback paused.")
}

// Stop playback
pub async fn playback_stop_endpoint(data: web::Data<Mutex<AppState>>) -> impl Responder {
    let control_tx = {
        let lock = data.lock().unwrap();
        lock.control_tx.clone()
    };

    if let Err(e) = control_tx.send(PlaybackCommand::Stop).await {
        return HttpResponse::InternalServerError()
            .body(format!("Error sending stop command: {}", e));
    }
    HttpResponse::Ok().body("Playback stopped.")
}

// Resume playback
pub async fn playback_resume_endpoint(data: web::Data<Mutex<AppState>>) -> impl Responder {
    let control_tx = {
        let lock = data.lock().unwrap();
        lock.control_tx.clone()
    };
    if let Err(e) = control_tx.send(PlaybackCommand::Resume).await {
        return HttpResponse::InternalServerError()
            .body(format!("Error sending resume command: {}", e));
    }
    HttpResponse::Ok().body("Playback resumed.")
}

// endregion: --- Region Title
