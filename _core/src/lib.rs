// region: --- Region Title
use surrealdb::Surreal;
pub mod io;
pub mod playback;
pub mod utils;
use playback::PlaybackCommand;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
// endregion: --- Region Title
// region: --- AppState

#[derive(Debug)]
pub struct AppState {
    pub playback_send: mpsc::Sender<PlaybackCommand>,
    pub current_sentence: Arc<Mutex<String>>,
    pub chat_db: Surreal<surrealdb::engine::local::Db>,
    pub audio_db: Surreal<surrealdb::engine::local::Db>,
}

impl Clone for AppState {
    fn clone(&self) -> Self {
        AppState {
            playback_send: self.playback_send.clone(),
            current_sentence: self.current_sentence.clone(),
            chat_db: self.chat_db.clone(),
            audio_db: self.audio_db.clone(),
        }
    }
}

// endregion: --- AppState
