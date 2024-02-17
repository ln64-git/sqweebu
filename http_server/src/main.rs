// region: --- imports
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use core::{AppState, _utils::playback};
use std::sync::Arc;
use tokio::sync::Mutex;
// endregion: --- imports

fn register_endpoints(cfg: &mut web::ServiceConfig) {
    cfg.route("/start", web::get().to(test_endpoint));
}
// nexus: web::Data<Arc<Mutex<AppState>>>
pub async fn test_endpoint() -> impl Responder {
    // let _ = start_counter(nexus.clone()).await;
    // let _ = speak_text("Hello World!", state.playback_send.clone()).await;

    // let state = nexus.lock().await;

    // let _ = speak_ollama(
    //     "What does the name Luke represent?".to_owned(),
    //     state.playback_send.clone(),
    // )
    // .await;

    HttpResponse::Ok().body("Test Complete.")
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    let nexus = Arc::new(Mutex::new(AppState {
        running: None,
        playback_send: playback::init_playback_channel().await,
        sentence_queue: Vec::new(),
    }));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(nexus.clone()))
            .configure(register_endpoints)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
