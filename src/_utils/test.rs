#![allow(dead_code)]

use super::azure::speak_text;
use crate::AppState;
use actix_web::{web, HttpResponse, Responder};
use std::sync::Mutex;

pub async fn test_endpoint(data: web::Data<Mutex<AppState>>) -> impl Responder {
    let playback_tx = {
        let lock = data.lock().unwrap();
        lock.playback_tx.clone()
    };

    // let recording_output_path = Path::new("temp/audio.wav");

    // record_audio(&recording_output_path).await;
    // // Process the resampled audio file with `speech_to_text`
    // speech_to_text(&recording_output_path).await;

    match speak_text("Hello World", playback_tx).await {
        Ok(_) => HttpResponse::Ok().body("Test complete."),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}
