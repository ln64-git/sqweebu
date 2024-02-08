use anyhow::{Context, Result};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use hound::{WavSpec, WavWriter};
use std::path::PathBuf;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::{Arc, Mutex};
use tokio::sync::Notify;

pub async fn record_audio(output_file_path: PathBuf, notify_stop: Arc<Notify>) -> Result<()> {
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
    let error_flag = Arc::new(AtomicBool::new(false));
    let stream = device.build_input_stream(
        &config,
        // Inside the stream setup, modify the callback function like so:
        move |data: &[f32], _: &cpal::InputCallbackInfo| {
            let mut writer_guard = writer_clone.lock().unwrap(); // Lock the mutex to access the writer
            if let Some(ref mut writer) = *writer_guard {
                // Dereference and pattern match to get the mutable reference to the WavWriter
                for &sample in data.iter() {
                    let sample_int = (sample * i16::MAX as f32) as i16; // Convert f32 sample to i16
                    if writer.write_sample(sample_int).is_err() {
                        // If writing sample fails, log error or handle it as needed
                        eprintln!("Failed to write sample");
                        break; // Exit the loop on error
                    }
                }
            }
        },
        |err| eprintln!("An error occurred on the recording stream: {}", err),
        None,
    )?;
    stream.play().context("Failed to play stream")?;

    let notify_future = notify_stop.notified();
    tokio::select! {
        _ = notify_future => {
            println!("Received stop signal.");
        }
        _ = tokio::time::sleep(tokio::time::Duration::from_secs(3600)), if !error_flag.load(Ordering::SeqCst) => {
            // This is a safeguard to stop recording after a certain time if no stop signal is received.
            // Adjust the duration according to your needs or remove if unnecessary.
            println!("Max recording duration reached.");
        }
    }

    drop(stream); // Stops the stream and recording

    // Ensure the writer is dropped outside of the lock to finalize it properly
    let mut writer_guard = writer.lock().unwrap();
    if let Some(writer) = writer_guard.take() {
        // Take the WavWriter out by replacing it with None
        writer.finalize().context("Failed to finalize WAV file")?;
    }

    println!("Recording stopped and saved.");

    Ok(())
}
