// src/main.rs

// region: --- modules
use actix_web::{web, App, Error, HttpResponse, HttpServer, Responder};
use futures::FutureExt;
use response_engine::{speak_clipboard, speak_ollama, AudioPlaybackManager};
use std::sync::Mutex;
// endregion: --- modules

// Utilize async for potential future asynchronous operations within the endpoints
async fn speak_clipboard_endpoint() -> impl Responder {
    match speak_clipboard().await {
        Ok(_) => HttpResponse::Ok().body("Spoke clipboard content"),
        Err(e) => HttpResponse::InternalServerError()
            .body(format!("Error speaking clipboard content: {}", e)),
    }
}

async fn speak_ollama_endpoint(
    body: web::Json<String>,
    data: web::Data<Mutex<AudioPlaybackManager>>,
) -> impl Responder {
    let preface = "In three sentences explain...";
    let final_prompt = format!("{} {}", preface, *body);
    match speak_ollama(final_prompt).await {
        Ok(audio_data) => {
            let mut manager = data.lock().unwrap();
            match manager.play_audio(audio_data) {
                Ok(_) => {
                    HttpResponse::Ok().body("Spoke generated Ollama content and started playback")
                }
                Err(e) => HttpResponse::InternalServerError().body(format!(
                    "Error generating Ollama content or starting playback: {}",
                    e
                )),
            }
        }
        Err(e) => HttpResponse::InternalServerError()
            .body(format!("Error generating Ollama content: {}", e)),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let audio_manager = AudioPlaybackManager::new();
        App::new()
            .app_data(web::Data::new(Mutex::new(audio_manager)))
            // Add routes for play, pause, resume, stop
            // .route("/play", web::post().to(play_endpoint))
            // .route("/pause", web::post().to(pause_endpoint))
            // .route("/resume", web::post().to(resume_endpoint))
            // .route("/stop", web::post().to(stop_endpoint))
            .route("/speak_clipboard", web::get().to(speak_clipboard_endpoint))
            .route("/speak_ollama", web::post().to(speak_ollama_endpoint))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
