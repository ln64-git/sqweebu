// region: --- Region Title
use surrealdb::Surreal;
pub mod io;
pub mod playback;
pub mod utils;
use playback::PlaybackCommand;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use utils::AudioEntry;
// endregion: --- Region Title
// region: --- AppState

#[derive(Debug)]
pub struct AppState {
    pub playback_send: mpsc::Sender<PlaybackCommand>,
    pub current_entry: Arc<Mutex<Option<AudioEntry>>>,
    pub chat_db: Surreal<surrealdb::engine::local::Db>,
    pub audio_db: Surreal<surrealdb::engine::local::Db>,
}

impl Clone for AppState {
    fn clone(&self) -> Self {
        AppState {
            playback_send: self.playback_send.clone(),
            current_entry: self.current_entry.clone(),
            chat_db: self.chat_db.clone(),
            audio_db: self.audio_db.clone(),
        }
    }
}

// endregion: --- AppState
