mod _api;
mod _utils;

// region: --- crates
pub use crate::_api::azure::azure_response_to_audio;
pub use crate::_api::azure::get_azure_response;
pub use crate::_api::ollama::ollama_generate_api;
pub use crate::_api::ollama::speak_ollama;
pub use crate::_utils::audio::speak_text;
pub use crate::_utils::clipboard::get_clipboard;
pub use crate::_utils::clipboard::speak_clipboard;
pub use crate::_utils::endpoints::pause_audio_endpoint;
pub use crate::_utils::endpoints::resume_audio_endpoint;
pub use crate::_utils::endpoints::speak_clipboard_endpoint;
pub use crate::_utils::endpoints::speak_ollama_endpoint;
pub use crate::_utils::endpoints::stop_audio_endpoint;
// endregion: --- crates

// region: --- modules
use _utils::endpoints;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use response_engine::{AppState, AudioPlaybackManager, PlaybackCommand};
use std::sync::Mutex;
use tokio::sync::mpsc;
use tokio::task::LocalSet;
// endregion: --- modules

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let local = LocalSet::new();
    let (tx, mut rx) = mpsc::channel::<PlaybackCommand>(32);

    local.spawn_local(async move {
        let mut audio_manager = AudioPlaybackManager::new();
        while let Some(command) = rx.recv().await {
            match audio_manager.handle_command(command).await {
                Ok(_) => {}
                Err(e) => eprintln!("Error executing audio command: {}", e),
            }
        }
    });

    let server_future = HttpServer::new(move || {
        let app_state = AppState { tx: tx.clone() };

        App::new()
            .app_data(web::Data::new(Mutex::new(app_state)))
            // Existing endpoints
            .route(
                "/speak_clipboard",
                web::get().to(endpoints::speak_clipboard_endpoint),
            )
            .route(
                "/speak_ollama",
                web::post().to(endpoints::speak_ollama_endpoint),
            )
            .route(
                "/pause/{id}",
                web::post().to(endpoints::pause_audio_endpoint),
            )
            .route(
                "/resume/{id}",
                web::post().to(endpoints::resume_audio_endpoint),
            )
            .route("/stop/{id}", web::post().to(endpoints::stop_audio_endpoint))
    })
    .bind("127.0.0.1:8080")?
    .run();

    local.run_until(server_future).await
}
