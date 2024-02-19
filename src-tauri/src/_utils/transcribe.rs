// region: --- Imports
use anyhow::{Context, Result};
use serde_json::Value;
use std::path::{Path, PathBuf};
use vosk::{CompleteResult, Model, Recognizer};
// endregion: --- Imports

fn serialize_complete_result(result: &CompleteResult) -> Result<String> {
    Ok(serde_json::to_string(&result).context("Failed to serialize CompleteResult")?)
}

pub async fn speech_to_text(audio_file_path: &Path) -> Result<String> {
    let model_path = PathBuf::from("/home/lucian/Documents/Models/vosk-model-small-en-us-0.15");
    let model_path_str = model_path.display().to_string();

    let model = Model::new(&model_path_str).context("Failed to load model")?;

    let mut recognizer = Recognizer::new(&model, 44100.0).context("Failed to create recognizer")?;

    let mut reader = hound::WavReader::open(audio_file_path).context("Failed to open WAV file")?;
    let samples: Vec<i16> = reader.samples().filter_map(Result::ok).collect();

    for chunk in samples.chunks(400) {
        recognizer.accept_waveform(chunk);
    }

    let result = recognizer.final_result();
    let result_str = serialize_complete_result(&result)?;
    let result_json: Value =
        serde_json::from_str(&result_str).context("Failed to parse result JSON")?;

    let transcription = result_json["text"].as_str().unwrap_or_default().to_string();

    Ok(transcription)
}
