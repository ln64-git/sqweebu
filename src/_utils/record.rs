// region: --- Modules
use crate::AudioRecordingManager;
use crate::RecordingCommand;
use anyhow::{Context, Result};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use hound::{WavSpec, WavWriter};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use tokio;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};
// endregion: --- Modules

/// Initializes the recording channel and spawns the recording thread.
pub async fn init_recording_channel() -> Sender<RecordingCommand> {
    let (record_tx, record_rx) = mpsc::channel::<RecordingCommand>(32);

    tokio::spawn(async move {
        recording_thread(record_rx).await;
    });

    record_tx
}

/// The recording thread function that processes recording commands.
async fn recording_thread(mut record_rx: Receiver<RecordingCommand>) {
    let recording_manager = AudioRecordingManager::new();

    while let Some(command) = record_rx.recv().await {
        match command {
            RecordingCommand::Start(_) => {
                recording_manager
                    .start_recording()
                    .await
                    .expect("Failed to start recording");
            }
            RecordingCommand::Stop => {
                recording_manager
                    .stop_recording()
                    .await
                    .expect("Failed to stop recording");
            }
        }
    }
}

pub async fn record_audio(output_file_path: PathBuf, is_recording: Arc<AtomicBool>) -> Result<()> {
    let host = cpal::default_host();
    let device = host
        .default_input_device()
        .context("No input device available")?;
    let default_config = device
        .default_input_config()
        .context("Failed to get default input config")?;
    let config = cpal::StreamConfig {
        channels: 1,
        sample_rate: default_config.sample_rate(),
        buffer_size: cpal::BufferSize::Default,
    };
    let spec = WavSpec {
        channels: 1,
        sample_rate: default_config.sample_rate().0,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let writer = Arc::new(Mutex::new(Some(
        WavWriter::create(output_file_path, spec).context("Failed to create WAV writer")?,
    )));
    let writer_clone = writer.clone();

    let is_recording_clone = is_recording.clone();

    let stream = device.build_input_stream(
        &config,
        move |data: &[f32], _: &cpal::InputCallbackInfo| {
            if is_recording_clone.load(Ordering::SeqCst) {
                let mut writer_guard = writer_clone.lock().unwrap();
                if let Some(ref mut writer) = *writer_guard {
                    for &sample in data.iter() {
                        let sample_int = (sample * i16::MAX as f32) as i16;
                        if writer.write_sample(sample_int).is_err() {
                            eprintln!("Failed to write sample");
                            break;
                        }
                    }
                }
            }
        },
        |err| eprintln!("An error occurred on the recording stream: {}", err),
        None,
    )?;
    stream.play().context("Failed to play stream")?;

    // Use is_recording flag to control the recording loop
    while is_recording.load(Ordering::SeqCst) {
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }

    drop(stream); // Stops the stream and recording

    let mut writer_guard = writer.lock().unwrap();
    if let Some(writer) = writer_guard.take() {
        writer.finalize().context("Failed to finalize WAV file")?;
    }

    println!("Recording stopped and saved.");

    Ok(())
}
