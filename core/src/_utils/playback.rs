use std::sync::Arc;
use std::thread;

use std::error::Error;
use tokio::runtime::Runtime;
use tokio::sync::mpsc::{self, Sender};
use tokio::sync::Mutex;

use crate::_utils::azure::speak_text;
use crate::{AppState, PlaybackCommand, PlaybackManager};

pub async fn ollama_playback_queue(nexus: Arc<Mutex<AppState>>) -> Result<(), Box<dyn Error>> {
    let mut nexus_lock = nexus.lock().await;
    let mut sentence_map_inner = nexus_lock.sentence_map.lock().await; // Lock the sentence map directly

    // Sort the keys in ascending order
    let mut keys: Vec<_> = sentence_map_inner.keys().cloned().collect();
    keys.sort();

    // Clone the playback_send to avoid moving it out of the MutexGuard
    let playback_send = nexus_lock.playback_send.clone();

    // Iterate over the sorted keys and print the corresponding values
    for key in keys {
        if let Some(sentence) = sentence_map_inner.get(&key) {
            speak_text(&sentence, &playback_send).await?; // Pass a reference to playback_send
            println!("{}: {}", key, sentence);
            // Remove the index from the sentence_map after speaking the value
            sentence_map_inner.remove(&key);
        }
    }

    Ok(())
}

pub async fn init_playback_channel() -> Sender<PlaybackCommand> {
    let (playback_tx, playback_rx) = mpsc::channel::<PlaybackCommand>(32);
    let (queue_tx, queue_rx) = mpsc::channel::<PlaybackCommand>(32);

    tokio::spawn(playback_control_thread(playback_rx, queue_tx.clone()));

    queued_playback_thread(queue_rx);

    playback_tx
}

async fn playback_control_thread(
    mut rx: mpsc::Receiver<PlaybackCommand>,
    queue_tx: mpsc::Sender<PlaybackCommand>,
) {
    while let Some(command) = rx.recv().await {
        // Forward commands to the third thread for queued playback
        let _ = queue_tx.send(command).await;
    }
}

fn queued_playback_thread(mut queue_rx: mpsc::Receiver<PlaybackCommand>) {
    thread::spawn(move || {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let mut audio_manager = PlaybackManager::new();
            while let Some(command) = queue_rx.recv().await {
                audio_manager.command_queue.push_back(command);
                if audio_manager
                    .is_idle
                    .load(std::sync::atomic::Ordering::SeqCst)
                {
                    audio_manager
                        .is_idle
                        .store(false, std::sync::atomic::Ordering::SeqCst);
                    audio_manager.start_processing_commands().await;
                    audio_manager
                        .is_idle
                        .store(true, std::sync::atomic::Ordering::SeqCst);
                }
            }
        });
    });
}
