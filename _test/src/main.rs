use _core::speak_text;
use _interface::init_playback_channel;
use chrono::Utc;

#[tokio::main]
async fn main() {
    env_logger::init();
    let start_time = Utc::now();
    std::env::set_var("RUST_LOG", "actix_web=debug");

    const TEXT: &str = "say something";

    let playback_send = init_playback_channel().await;
    let _ = speak_text(TEXT, "google", &playback_send).await;
    println!("Test Complete!");

    let end_time = Utc::now(); // Record end time
    let duration = end_time.signed_duration_since(start_time); // Calculate duration
    let seconds = duration.num_seconds();
    println!("Execution time: {} seconds", seconds);
}
