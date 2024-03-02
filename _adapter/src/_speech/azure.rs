// src/_utils/azure.rs

// region: --- Imports

use std::error::Error;

// endregion: --- Imports

pub async fn get_azure_speech_response(
    text_to_speak: &str,
    api_key: &str,
    region: &str,
    voice_gender: &str,
    voice_name: &str,
    output_format: &str,
) -> Result<reqwest::Response, Box<dyn Error>> {
    let token_url = format!(
        "https://{}.api.cognitive.microsoft.com/sts/v1.0/issueToken",
        region
    );
    let tts_url = format!(
        "https://{}.tts.speech.microsoft.com/cognitiveservices/v1",
        region
    );

    let token_response = reqwest::Client::new()
        .post(&token_url)
        .header("Ocp-Apim-Subscription-Key", api_key)
        .header("Content-Length", "0")
        .send()
        .await?;
    let access_token = token_response.text().await?;

    let tts_response = reqwest::Client::new()
        .post(&tts_url)
        .header("Authorization", format!("Bearer {}", access_token))
        .header("Content-Type", "application/ssml+xml")
        .header("X-Microsoft-OutputFormat", output_format)
        .header("User-Agent", "text-to-speech-exp")
        .body(format!(
            r#"<speak version='1.0' xml:lang='en-US'><voice xml:lang='en-US' xml:gender='{}' name='{}'>{}</voice></speak>"#,
            voice_gender, voice_name, text_to_speak
        ))
        .send()
        .await?;

    Ok(tts_response)
}

// let region = "eastus";
// let voice_gender = "Female";
// let voice_name = "en-US-JennyNeural";
// let output_format = "audio-48khz-192kbitrate-mono-mp3";

// let audio_content = tts_response.bytes().await?;
// Ok(audio_content.into_iter().collect())
