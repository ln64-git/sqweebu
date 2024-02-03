// src/utils/endpoints.rs

use actix_web::{web, HttpResponse, Responder};
use std::sync::Mutex;

use crate::{speak_clipboard, speak_ollama, AppState};

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
