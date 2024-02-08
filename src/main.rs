// src/main.rs

use response_engine::launch_playback_server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    launch_playback_server().await
}
