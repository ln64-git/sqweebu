//  src/_utils/transcribe.rs

use anyhow::Result;
use std::{env, path::Path};
use vosk::{Model, Recognizer}; // Assuming you're using the anyhow crate for error handling

pub async fn speech_to_text(audio_file_path: &Path) -> Result<()> {
    let model_path =
        env::var("VOSK_MODEL_PATH").expect("VOSK_MODEL_PATH must be set in the .env file");
    let model = Model::new(&model_path).expect("Failed to load model");

    // Initialize the recognizer with additional settings
    let mut recognizer = Recognizer::new(&model, 44100.0).expect("Failed to create recognizer");
    recognizer.set_max_alternatives(10);
    recognizer.set_words(true);
    recognizer.set_partial_words(true);

    // Load audio data from file
    let mut reader = hound::WavReader::open(audio_file_path)?;
    let samples: Vec<i16> = reader.samples().filter_map(Result::ok).collect();

    // Process the audio data in chunks
    for chunk in samples.chunks(400) {
        recognizer.accept_waveform(chunk);
    }

    // Output the final result
    println!("{:#?}", recognizer.final_result());
    Ok(())
}
