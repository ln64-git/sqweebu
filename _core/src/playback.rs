// src/_utils/playback.rs

// region: --- importswWE
use _interface::{PlaybackCommand, PlaybackManager};
use rodio::{OutputStream, Sink};
use tokio::sync::mpsc::{self, Sender};
// endregion: --- imports

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
            let (_stream, stream_handle) = OutputStream::try_default().unwrap();
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
