// region: --- Region Title
pub mod playback;
use _interface::{get_sentence_from_api, get_speech_from_api};
use playback::PlaybackCommand;
use std::error::Error;
use tokio::sync::mpsc;
// endregion: --- Region Title

#[derive(Debug)]
pub struct AppState {
    pub playback_send: mpsc::Sender<PlaybackCommand>,
}

impl Clone for AppState {
    fn clone(&self) -> Self {
        AppState {
            playback_send: self.playback_send.clone(),
        }
    }
}

use surrealdb::engine::local::RocksDb;
use surrealdb::sql::Thing;
use surrealdb::Surreal;
use tauri_api::path::data_dir;

pub async fn process_input(
    text: &str,
    playback_send: &mpsc::Sender<PlaybackCommand>,
) -> Result<(), Box<dyn Error>> {
    let mut input_text = text.to_owned();

    let _ = match text {
        input if input.starts_with("speak text") => {
            input_text = input[10..].to_owned(); // Store the text without the "speak text" prefix
            speak_text(&input[10..], "azure", playback_send).await
        }
        input if input.starts_with("speak gpt") => {
            input_text = input[9..].to_owned(); // Store the text without the "speak gpt" prefix
            speak_gpt((&input[9..]).to_owned(), "ollama", "azure", playback_send).await
        }
        _ => Ok(()),
    };

    // Create a RocksDB database connection
    let data_dir = data_dir().unwrap(); // This assumes the operation won't fail

    // Create a RocksDB database connection using the Tauri configuration directory
    let db_path = data_dir.join("database");
    let db = Surreal::new::<RocksDb>(db_path.to_str().unwrap()).await?; // Create database connection

    // Select a namespace and database
    db.use_ns("user").use_db("chat").await?;

    // Create a new record with the input and result
    let created: Vec<Thing> = db.create("input").content(input_text).await?; // Pass input text as content

    // Ensure only one record is created, as per your requirement
    let _ = created.into_iter().next();

    Ok(())
}

pub async fn speak_text(
    text: &str,
    speech_service: &str,
    playback_send: &mpsc::Sender<PlaybackCommand>,
) -> Result<(), Box<dyn Error>> {
    let audio_data = get_speech_from_api(text, speech_service).await?;
    let _ = playback_send.send(PlaybackCommand::Play(audio_data)).await;
    Ok(())
}

pub async fn speak_gpt(
    text: String,
    gpt_service: &str,
    speech_service: &str,
    playback_send: &mpsc::Sender<PlaybackCommand>,
) -> Result<(), Box<dyn Error>> {
    let (sentence_send, mut sentence_recv) = mpsc::channel::<String>(32);
    let gpt_service_cloned = gpt_service.to_string();
    tokio::spawn(async move {
        match get_sentence_from_api(text.clone(), &gpt_service_cloned, sentence_send).await {
            Ok(_) => {}
            Err(e) => eprintln!("Failed to generate sentences: {}", e),
        }
    });

    while let Some(sentence) = sentence_recv.recv().await {
        speak_text(&sentence, speech_service, &playback_send).await?;
    }
    Ok(())
}
