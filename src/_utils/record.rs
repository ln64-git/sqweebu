// //  src/_utils/record.rs

// use anyhow::{Context, Result};
// use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
// use hound::{WavSpec, WavWriter};
// use std::{
//     io::{self, BufRead},
//     path::Path,
//     sync::{Arc, Mutex},
// };

// pub async fn record_audio(output_file_path: &Path) -> Result<()> {
//     let host = cpal::default_host();
//     let device = host
//         .default_input_device()
//         .context("No input device available")?;

//     let default_config = device
//         .default_input_config()
//         .context("Failed to get default input config")?;
//     let config = cpal::StreamConfig {
//         channels: 1,
//         sample_rate: default_config.sample_rate(),
//         buffer_size: cpal::BufferSize::Default,
//     };

//     let spec = WavSpec {
//         channels: 1,
//         sample_rate: default_config.sample_rate().0,
//         bits_per_sample: 16,
//         sample_format: hound::SampleFormat::Int,
//     };
//     let writer = Arc::new(Mutex::new(Some(
//         WavWriter::create(output_file_path, spec).context("Failed to create WAV writer")?,
//     )));

//     let writer_clone = Arc::clone(&writer);
//     let stream = device.build_input_stream(
//         &config,
//         move |data: &[f32], _: &cpal::InputCallbackInfo| {
//             let mut writer = writer_clone.lock().unwrap();
//             if let Some(writer) = writer.as_mut() {
//                 for &sample in data.iter() {
//                     writer
//                         .write_sample((sample * i16::MAX as f32) as i16)
//                         .expect("Failed to write sample");
//                 }
//             }
//         },
//         |err| eprintln!("An error occurred on the recording stream: {}", err),
//         None,
//     )?;

//     stream.play().context("Failed to play stream")?;
//     println!("Recording... Press Enter to stop.");
//     io::stdin().lock().read_line(&mut String::new())?;

//     drop(stream); // Stops recording

//     // Take the writer out of the mutex to finalize it
//     let maybe_writer = writer.lock().unwrap().take();
//     if let Some(wav_writer) = maybe_writer {
//         wav_writer
//             .finalize()
//             .context("Failed to finalize WAV file")?;
//     }

//     println!("Recording stopped and saved to {:?}", output_file_path);

//     Ok(())
// }
