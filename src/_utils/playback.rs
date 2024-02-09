use std::thread;

use tokio::runtime::Runtime;
use tokio::sync::mpsc::{self, Sender};

use crate::{PlaybackCommand, PlaybackManager};

pub async fn init_playback_channel() -> Sender<PlaybackCommand> {
    let (playback_tx, playback_rx) = mpsc::channel::<PlaybackCommand>(32);
    let (queue_tx, queue_rx) = mpsc::channel::<PlaybackCommand>(32);

    // Correctly spawn the Playback Control Thread as it is async
    tokio::spawn(playback_control_thread(playback_rx, queue_tx.clone()));

    // Directly call queued_playback_thread without tokio::spawn
    queued_playback_thread(queue_rx);

    playback_tx
}

// Playback Control Thread
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
