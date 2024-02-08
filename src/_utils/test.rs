use super::azure::speak_text;
use crate::AppState;
use actix_web::{web, HttpResponse, Responder};
use std::sync::Mutex;

pub async fn test_endpoint(data: web::Data<Mutex<AppState>>) -> impl Responder {
    // First, clone the data you need out of the app_state
    let (playback_tx, transcribed_text) = {
        let app_state_lock = data.lock().unwrap(); // Lock the mutex to access the AppState
        (
            app_state_lock.playback_tx.clone(),
            app_state_lock.transcribed_text.clone(),
        ) // Clone the playback_tx and transcribed_text
    }; // MutexGuard is dropped here because it's out of scope

    println!("Transcribed text: {:?}", transcribed_text);
    // Now, you can check if there's transcribed text available without holding onto the lock
    if let Some(text) = transcribed_text {
        // Text is available, so we pass it to speak_text
        match speak_text(&text, playback_tx).await {
            Ok(_) => HttpResponse::Ok().body("Test complete with transcribed text."),
            Err(e) => {
                HttpResponse::InternalServerError().body(format!("Error speaking text: {}", e))
            }
        }
    } else {
        // No transcribed text is available
        HttpResponse::Ok().body("No transcribed text available.")
    }
}
