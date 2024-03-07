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

use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;
use surrealdb::Surreal;

use std::error::Error;
use surrealdb::engine::local::Mem;
use surrealdb::kvp;
use surrealdb::Surreal;
use surrealdb::Thing;

use std::error::Error;
use surrealdb::engine::local::RocksDB;
use surrealdb::kvp;
use surrealdb::Surreal;
use surrealdb::Thing;

pub async fn process_input(
    text: &str,
    playback_send: &mpsc::Sender<PlaybackCommand>,
) -> Result<(), Box<dyn Error>> {
    let mut result: Result<(), Box<dyn Error>> = Ok(());
    let mut input_to_store = text.to_owned();

    result = match text {
        input if input.starts_with("speak text") => {
            input_to_store = input[10..].to_owned(); // Store the text without the "speak text" prefix
            speak_text(&input[10..], "azure", playback_send).await
        }
        input if input.starts_with("speak gpt") => {
            input_to_store = input[9..].to_owned(); // Store the text without the "speak gpt" prefix
            speak_gpt((&input[9..]).to_owned(), "ollama", "azure", playback_send).await
        }
        _ => Ok(()),
    };

    // Create a RocksDB database connection
    let db = Surreal::new::<RocksDB>(()).await?;

    // Select a namespace and database
    db.use_ns("guest_chat").use_db("guest_chat").await?;

    // Create a new record with the input and result
    let created: Option<Thing> = db
        .create("records")
        .content((
            kvp!("input", input_to_store),
            kvp!("result", result.is_ok().to_string()),
        ))
        .await?;

    result
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
