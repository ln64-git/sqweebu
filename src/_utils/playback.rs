use tokio::runtime::Runtime;
use tokio::sync::mpsc::{self, Sender};

use crate::{PlaybackCommand, PlaybackManager};

pub async fn init_playback_channel() -> Sender<PlaybackCommand> {
    let (playback_tx, playback_rx) = mpsc::channel::<PlaybackCommand>(32);
    let (queue_tx, queue_rx) = mpsc::channel::<PlaybackCommand>(32);

    tokio::spawn(async move {
        playback_thread(playback_rx, queue_tx.clone()).await;
    });

    std::thread::spawn(move || {
        playback_queue_thread(queue_rx);
    });

    playback_tx
}

async fn playback_thread(
    mut playback_rx: mpsc::Receiver<PlaybackCommand>,
    queue_tx: mpsc::Sender<PlaybackCommand>,
) {
    while let Some(command) = playback_rx.recv().await {
        let _ = queue_tx.send(command).await;
    }
}

fn playback_queue_thread(mut queue_rx: mpsc::Receiver<PlaybackCommand>) {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let mut playback_manager = PlaybackManager::new();
        while let Some(command) = queue_rx.recv().await {
            playback_manager.command_queue.push_back(command);
            if playback_manager
                .is_idle
                .load(std::sync::atomic::Ordering::SeqCst)
            {
                playback_manager
                    .is_idle
                    .store(false, std::sync::atomic::Ordering::SeqCst);
                playback_manager.start_processing_commands().await;
                playback_manager
                    .is_idle
                    .store(true, std::sync::atomic::Ordering::SeqCst);
            }
        }
    });
}
