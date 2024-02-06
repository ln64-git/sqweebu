use super::azure::speak_text;
use crate::AppState;
use actix_web::{web, HttpResponse, Responder};
use std::sync::Mutex;

pub async fn test_endpoint(data: web::Data<Mutex<AppState>>) -> impl Responder {
    let control_tx = {
        let lock = data.lock().unwrap();
        lock.control_tx.clone()
    };
    match speak_text("Hello World", control_tx).await {
        Ok(_) => HttpResponse::Ok().body("Test complete."),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}
