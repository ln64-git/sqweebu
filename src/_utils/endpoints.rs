// src/_utils/endpoints.rs

use actix_web::{web, HttpResponse, Responder};
use std::{path::PathBuf, sync::Mutex};

use crate::{speak_clipboard, speak_ollama, AppState, PlaybackCommand, RecordingCommand};

// region: --- Main Endpoints
pub async fn speak_clipboard_endpoint(data: web::Data<Mutex<AppState>>) -> impl Responder {
    let playback_tx = {
        let lock = data.lock().unwrap();
        lock.playback_tx.clone()
    };

    match speak_clipboard(playback_tx).await {
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

    let playback_tx = {
        let lock = data.lock().unwrap();
        lock.playback_tx.clone()
    };

    match speak_ollama(final_prompt, playback_tx).await {
        Ok(_) => HttpResponse::Ok().body("Ollama content spoken."),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}
// endregion: --- Main Endpoints

// region: --- Recording Endpoints

pub async fn record_start_endpoint(data: web::Data<Mutex<AppState>>) -> impl Responder {
    let record_tx = {
        let lock = data.lock().unwrap();
        lock.record_tx.clone()
    };
    // referemce temp path
    let path = PathBuf::from("/temp/recording.wav");

    // Assuming the path is predetermined or not needed
    if let Err(e) = record_tx.send(RecordingCommand::Start(path)).await {
        // Adjusted for simplicity
        println!("Error sending start recording command: {}", e);
        return HttpResponse::InternalServerError()
            .body(format!("Error sending start recording command: {}", e));
    }

    HttpResponse::Ok().body("Recording start command sent.")
}

pub async fn record_stop_endpoint(data: web::Data<Mutex<AppState>>) -> impl Responder {
    // Extract the recording manager's sender from AppState
    let record_tx = {
        let lock = data.lock().unwrap();
        lock.record_tx.clone()
    };

    // Send the Stop command through the channel
    if let Err(e) = record_tx.send(RecordingCommand::Stop).await {
        println!("Error sending stop recording command: {}", e);
        return HttpResponse::InternalServerError()
            .body(format!("Error sending stop recording command: {}", e));
    }
    HttpResponse::Ok().body("Recording stop command sent.")
}

// endregion: --- Recording Endpoints

// region: --- Playback Endpoints

pub async fn playback_pause_endpoint(
    data: web::Data<Mutex<AppState>>,
    id: web::Path<usize>,
) -> impl actix_web::Responder {
    let tx = data.lock().unwrap().playback_tx.clone();
    if let Err(_) = tx.send(PlaybackCommand::Pause(*id)).await {
        return HttpResponse::InternalServerError().body("Failed to send pause command.");
    }
    HttpResponse::Ok().body(format!("Pause command sent for ID: {}", id))
}

pub async fn playback_resume_endpoint(
    data: web::Data<Mutex<AppState>>,
    id: web::Path<usize>,
) -> impl actix_web::Responder {
    let tx = data.lock().unwrap().playback_tx.clone();
    if let Err(_) = tx.send(PlaybackCommand::Resume(*id)).await {
        return HttpResponse::InternalServerError().body("Failed to send resume command.");
    }
    HttpResponse::Ok().body("Resume command sent.")
}

pub async fn playback_stop_endpoint(
    data: web::Data<Mutex<AppState>>,
    id: web::Path<usize>,
) -> impl actix_web::Responder {
    let playback_tx = data.lock().unwrap().playback_tx.clone();
    if let Err(_) = playback_tx.send(PlaybackCommand::Stop(*id)).await {
        return HttpResponse::InternalServerError().body("Failed to send stop command.");
    }
    HttpResponse::Ok().body("Stop command sent.")
}

// endregion: --- Region Title
