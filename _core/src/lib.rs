// region: --- Region Title
pub mod playback;
use _interface::{ get_sentence_from_gpt, get_speech_from_api };
use playback::PlaybackCommand;
use serde::{ Deserialize, Serialize };
use std::error::Error;
use chrono::{ DateTime, Utc };
use tokio::sync::mpsc;
// endregion: --- Region Title

#[derive(Debug)]
pub struct AppState {
    pub playback_send: mpsc::Sender<PlaybackCommand>,
    pub db: Surreal<surrealdb::engine::local::Db>,
}

impl Clone for AppState {
    fn clone(&self) -> Self {
        AppState {
            playback_send: self.playback_send.clone(),
            db: self.db.clone(), // Clone the database connection as well
        }
    }
}

use surrealdb::Surreal;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ChatEntry {
    timestamp: DateTime<Utc>,
    body: String,
}

pub async fn process_input(
    text: &str,
    playback_send: &mpsc::Sender<PlaybackCommand>,
    db: Surreal<surrealdb::engine::local::Db>
) -> Result<(), Box<dyn Error>> {
    let mut input_text = text.to_owned();

    let _ = match text {
        input if input.starts_with("speak text") => {
            input_text = input[10..].to_owned(); // Store the text without the "speak text" prefix
            speak_text(&input[10..], "azure", playback_send).await
        }
        input if input.starts_with("speak gpt") => {
            input_text = input[9..].to_owned(); // Store the text without the "speak gpt" prefix
            speak_gpt((&input[9..]).to_owned(), "ollama", db.clone()).await
        }
        _ => Ok(()),
    };

    db.use_ns("user").use_db("user").await?;

    let content = ChatEntry {
        timestamp: Utc::now(),
        body: input_text,
    };

    let _: Option<Vec<ChatEntry>> = match db.create("chat").content(content).await {
        Ok(records) => {
            records.clone().into_iter().next();
            Some(records)
        }
        Err(e) => {
            println!("PROCESS_INPUT - Error: {:?}", e);
            None
        }
    };
    let _: Vec<ChatEntry> = db.select("chat").await?;

    Ok(())
}

pub async fn process_response(
    sentence: String,
    db: Surreal<surrealdb::engine::local::Db>
) -> Result<(), Box<dyn Error>> {
    db.use_ns("user").use_db("user").await?;

    let content = ChatEntry {
        timestamp: Utc::now(),
        body: sentence,
    };

    let _: Option<Vec<ChatEntry>> = match db.create("chat").content(content).await {
        Ok(records) => {
            records.clone().into_iter().next();
            Some(records)
        }
        Err(e) => {
            println!("PROCESS_INPUT - Error: {:?}", e);
            None
        }
    };
    let _: Vec<ChatEntry> = db.select("chat").await?;

    Ok(())
}

pub async fn speak_text(
    text: &str,
    speech_service: &str,
    playback_send: &mpsc::Sender<PlaybackCommand>
) -> Result<(), Box<dyn Error>> {
    let audio_data = get_speech_from_api(text, speech_service).await?;
    let _ = playback_send.send(PlaybackCommand::Play(audio_data)).await;
    Ok(())
}

pub async fn speak_gpt(
    text: String,
    gpt_service: &str,
    db: Surreal<surrealdb::engine::local::Db>
    // speech_service: &str,
    // playback_send: &mpsc::Sender<PlaybackCommand>,
) -> Result<(), Box<dyn Error>> {
    let (sentence_send, mut sentence_recv) = mpsc::channel::<String>(32);
    let gpt_service_cloned = gpt_service.to_string();
    tokio::spawn(async move {
        match get_sentence_from_gpt(text.clone(), &gpt_service_cloned, sentence_send).await {
            Ok(_) => {}
            Err(e) => eprintln!("Failed to generate sentences: {}", e),
        }
    });

    while let Some(sentence) = sentence_recv.recv().await {
        let _ = process_response(sentence, db.clone()).await;
        // speak_text(&sentence, speech_service, &playback_send).await?;
    }
    Ok(())
}
