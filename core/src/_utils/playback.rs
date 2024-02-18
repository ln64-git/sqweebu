use std::sync::Arc;
use std::thread;

use rodio::{OutputStream, Sink};
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
    let (playback_send, playback_recv) = mpsc::channel::<PlaybackCommand>(32);
    let (queue_send, queue_recv) = mpsc::channel::<PlaybackCommand>(32);

    tokio::spawn(playback_control_thread(playback_recv, queue_send.clone()));

    queued_playback_thread(queue_recv);

    playback_send
}

async fn playback_control_thread(
    mut playback_recv: mpsc::Receiver<PlaybackCommand>,
    queue_send: mpsc::Sender<PlaybackCommand>,
) {
    while let Some(command) = playback_recv.recv().await {
        let _ = queue_send.send(command).await;
    }
}

fn queued_playback_thread(mut queue_recv: mpsc::Receiver<PlaybackCommand>) {
    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let atomic_order = std::sync::atomic::Ordering::SeqCst;
        rt.block_on(async {
            let (stream, stream_handle) = OutputStream::try_default().unwrap();
            let sink = Sink::try_new(&stream_handle).unwrap();

            let mut playback = PlaybackManager::new(sink);
            while let Some(command) = queue_recv.recv().await {
                playback.command_queue.push_back(command);
                if playback.is_idle.load(atomic_order) {
                    playback.is_idle.store(false, atomic_order);
                    playback.process_command_queue().await;
                    playback.is_idle.store(true, atomic_order);
                }
            }
        });
    });
}
