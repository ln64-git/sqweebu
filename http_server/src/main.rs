use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use core::{
    AppState, PlaybackCommand,
    _utils::{ollama::speak_ollama, playback},
};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

fn register_endpoints(cfg: &mut web::ServiceConfig) {
    cfg.route("/test", web::get().to(test_endpoint))
        .route("/pause", web::get().to(pause_playback_endpoint))
        .route("/resume", web::get().to(resume_playback_endpoint))
        .route("/stop", web::get().to(stop_playback_endpoint));
}

async fn pause_playback_endpoint(data: web::Data<Arc<Mutex<AppState>>>) -> impl Responder {
    let state = data.lock().await;
    let playback_send = state.playback_send.to_owned();

    if let Err(e) = playback_send.send(PlaybackCommand::Pause).await {
        println!("Error sending pause command: {}", e);
        return HttpResponse::InternalServerError()
            .body(format!("Error sending pause command: {}", e));
    }
    println!("Playback endpoint paused.");
    HttpResponse::Ok().body("Playback paused.")
}

async fn stop_playback_endpoint(data: web::Data<Arc<Mutex<AppState>>>) -> impl Responder {
    let state = data.lock().await;
    let playback_send = state.playback_send.to_owned();

    if let Err(e) = playback_send.send(PlaybackCommand::Stop).await {
        return HttpResponse::InternalServerError()
            .body(format!("Error sending stop command: {}", e));
    }
    println!("Playback endpoint stopped.");

    HttpResponse::Ok().body("Playback stopped.")
}

async fn resume_playback_endpoint(data: web::Data<Arc<Mutex<AppState>>>) -> impl Responder {
    let state = data.lock().await;
    let playback_send = state.playback_send.to_owned();

    if let Err(e) = playback_send.send(PlaybackCommand::Resume).await {
        return HttpResponse::InternalServerError()
            .body(format!("Error sending resume command: {}", e));
    }

    HttpResponse::Ok().body("Playback resumed.")
}

pub async fn test_endpoint(data: web::Data<Arc<Mutex<AppState>>>) -> impl Responder {
    let nexus = data.get_ref().clone();

    let _ = speak_ollama(
        "Give me three sentences about the future?".to_owned(),
        nexus,
    )
    .await;

    HttpResponse::Ok().body("Test Complete.")
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    let nexus = Arc::new(Mutex::new(AppState {
        running: None,
        playback_send: playback::init_playback_channel().await,
        sentence_map: Arc::new(Mutex::new(HashMap::new())),
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
