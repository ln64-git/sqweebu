// region: --- Playback Manager

use _adapter::{azure::get_azure_speech_response, google::get_google_speech_response};
use dotenv::dotenv;
use std::{env, error::Error};

pub async fn get_speech_from_api(
    text: &str,
    speech_service: &str,
) -> Result<Vec<u8>, Box<dyn Error>> {
    dotenv().ok();
    match speech_service {
        "google" => {
            let text_to_speak = text;
            let language_code = "en-US";
            let api_key = env::var("GOOGLE_API_KEY").expect("GOOGLE_API_KEY not found");
            let response =
                get_google_speech_response(text_to_speak, language_code, &api_key).await?;
            let audio_content = response.bytes().await?;
            Ok(audio_content.to_vec())
        }
        "azure" => {
            let text_to_speak = text;
            let api_key = env::var("GOOGLE_API_KEY").expect("AZURE_API_KEY not found");
            let region = "eastus";
            let voice_gender = "Female";
            let voice_name = "en-US-JennyNeural";
            let output_format = "audio-48khz-192kbitrate-mono-mp3";
            let response = get_azure_speech_response(
                text_to_speak,
                &api_key,
                region,
                voice_gender,
                voice_name,
                output_format,
            )
            .await?;
            let audio_content = response.bytes().await?;
            Ok(audio_content.to_vec())
        }
        _ => Err("Invalid service specified".into()), // Handle invalid service
    }
}
