use reqwest::Client;
use std::error::Error;

/// Retrieves audio response from Google API using hexadecimal encoding.
pub async fn get_google_speech_response(
    text_to_speak: &str,
    language_code: &str,
    api_key: &str,
) -> Result<reqwest::Response, Box<dyn Error>> {
    // Define API endpoint
    let endpoint = "https://texttospeech.googleapis.com/v1/text:synthesize";

    // Create the request body JSON with the specified device profile
    let request_body = format!(
        r#"{{
            "input": {{"text": "{}"}},
            "voice": {{"languageCode": "{}"}},
            "audioConfig": {{
                "audioEncoding": "LINEAR16",
                "effectsProfileId": ["telephony-class-application"]
            }}
        }}"#,
        text_to_speak, language_code
    );

    // Send POST request to Google Cloud Text-to-Speech API
    let client = Client::new();
    let tts_response = client
        .post(endpoint)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .body(request_body)
        .send()
        .await?;

    // Extract audio content from response
    // let audio_content = tts_response.bytes().await?;

    Ok(tts_response)
}
