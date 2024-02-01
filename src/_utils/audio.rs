// region: --- modules
use bytes::Bytes;
use reqwest::Response;
use rodio::{Decoder, OutputStream, Sink, Source};
use std::error::Error;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::process;
use std::time::Duration;
use std::{fs::File, io::Write};
use tempfile::{tempdir, TempDir};
use tokio::fs::File as OtherFile;
// endregion: --- modules

pub async fn listen_to_audio_stream(response: Response) -> Result<(), Box<dyn Error>> {
    let audio_content = response.bytes().await?;
    let audio_content_slice = audio_content.as_ref();
    let (temp_dir, file_path) = save_audio_to_temp(audio_content_slice)?;
    listen_to_audio_file(&file_path)?;
    Ok(())
}

pub fn save_audio_to_temp(
    audio_content: &[u8],
) -> Result<(TempDir, String), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    let file_path = temp_dir.path().join("audio_content.wav");
    let mut file = File::create(&file_path)?;
    file.write_all(audio_content)?;
    // println!("Audio content saved to temporary location: {:?}", file_path);
    Ok((temp_dir, file_path.to_string_lossy().into_owned()))
}
pub fn listen_to_audio_file(file_path: &str) -> io::Result<()> {
    let (_stream, stream_handle) = OutputStream::try_default()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("{:?}", e)))?;

    let sink = Sink::try_new(&stream_handle)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("{:?}", e)))?;

    let file = File::open(file_path)?;
    let buf_reader = BufReader::new(file);
    let decoder = Decoder::new(buf_reader)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("{:?}", e)))?;

    sink.append(decoder);

    // Wait for the audio to finish playing
    sink.sleep_until_end();

    Ok(())
}
