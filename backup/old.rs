use actix_web::{web, App, HttpServer, Responder};
use response_engine::{pause, resume, speak_clipboard, speak_ollama, SpeechState};
use rodio::{Decoder, OutputStream, Sink, Source};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

pub async fn speak_ollama_api(
    data: web::Data<Arc<Mutex<SpeechState>>>, // The type of `data` is web::Data wrapping Arc<Mutex<SpeechState>>
) -> impl Responder {
    let prompt = "In three sentences explain async rust";
    let state_arc: Arc<Mutex<SpeechState>> = data.into_inner(); // Ensure no double wrapping
    tokio::spawn(async move {
        if let Err(e) = speak_ollama(prompt.to_owned(), state_arc).await {
            eprintln!("Error during playback: {}", e);
        }
    });

    "Speak Ollama API called"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let speech_state = Arc::new(Mutex::new(SpeechState::new()));
    let speech_state_data = web::Data::new(speech_state);

    HttpServer::new(move || {
        App::new()
            .app_data(speech_state_data.clone())
            .route("/pause", web::get().to(pause))
            .route("/resume", web::get().to(resume))
            .route("/speak_ollama", web::post().to(speak_ollama_api))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
