//  src/_utils/server.rs

// region: --- Modules
pub use crate::_utils::endpoints::playback_pause_endpoint;
pub use crate::_utils::endpoints::playback_resume_endpoint;
pub use crate::_utils::endpoints::playback_stop_endpoint;
pub use crate::_utils::endpoints::speak_clipboard_endpoint;
pub use crate::_utils::endpoints::speak_ollama_endpoint;
use crate::test_endpoint;
use crate::AppState;
use actix_web::{web, App, HttpServer};
use std::sync::Mutex;
use super::endpoints::record_start_endpoint;
use super::endpoints::record_stop_endpoint;
use super::playback;
use super::record;
// endregion: --- Modules

fn register_endpoints(cfg: &mut web::ServiceConfig) {
    cfg.route("/speak_clipboard", web::get().to(speak_clipboard_endpoint))
        .route("/speak_ollama", web::post().to(speak_ollama_endpoint))
        .route("/playback/pause", web::get().to(playback_pause_endpoint))
        .route("/playback/stop", web::get().to(playback_stop_endpoint))
        .route("/playback/resume", web::get().to(playback_resume_endpoint))
        .route("/record/start", web::get().to(record_start_endpoint))
        .route("/record/stop", web::get().to(record_stop_endpoint))
        .route("/test", web::get().to(test_endpoint));
}

pub async fn launch_playback_server() -> std::io::Result<()> {
    // Playback && Recording Channel setup
    let playback_tx = playback::init_playback_channel().await;
    let record_tx = record::init_recording_channel().await;

    // Server setup and start
    HttpServer::new(move || {
        let app_state = AppState {
            playback_tx: playback_tx.clone(),
            record_tx: record_tx.clone(),
            transcribed_text: None,
        };

        App::new()
            .app_data(web::Data::new(Mutex::new(app_state)))
            .configure(register_endpoints)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
