use std::sync::Arc;
// region: --- imports
use std::error::Error;
use std::thread;
use tokio::runtime::Runtime;
use tokio::sync::mpsc::{self, Sender};
use tokio::sync::Mutex;
// endregion: --- imports

use crate::{AppState, PlaybackCommand, PlaybackManager};

pub async fn ollama_playback_queue(nexus: Arc<Mutex<AppState>>) -> Result<(), Box<dyn Error>> {
    let nexus_lock = nexus.lock().await;
    let playback_map = nexus_lock.sentence_map.clone(); // Fetch the playback queue from the state

    // Print out the sentence map to check its contents
    println!("PART 3 - sentence map: {:#?}", playback_map);

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
        // Forward commands to the third thread for queued playback
        let _ = queue_send.send(command).await;
    }
}

fn queued_playback_thread(mut queue_recv: mpsc::Receiver<PlaybackCommand>) {
    thread::spawn(move || {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let mut playback = PlaybackManager::new();
            while let Some(command) = queue_recv.recv().await {
                playback.command_queue.push_back(command);
                if playback.is_idle.load(std::sync::atomic::Ordering::SeqCst) {
                    playback
                        .is_idle
                        .store(false, std::sync::atomic::Ordering::SeqCst);
                    playback.start_processing_commands().await;
                    playback
                        .is_idle
                        .store(true, std::sync::atomic::Ordering::SeqCst);
                }
            }
        });
    });
}
