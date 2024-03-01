use dotenv::dotenv;
use reqwest::Client;
use std::{env, error::Error};

/// Retrieves audio response from Google API using hexadecimal encoding.
pub async fn get_google_audio_response(text_to_speak: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    dotenv().ok();

    // Fetch Google API key from environment variables
    let api_key =
        env::var("GOOGLE_API_KEY").expect("GOOGLE_API_KEY not found in environment variables");

    // Define API endpoint
    let endpoint = "https://texttospeech.googleapis.com/v1/text:synthesize";

    // Define device profile for the audio
    let device_profile = "telephony-class-application"; // Example device profile

    // Create the request body JSON with the specified device profile
    let request_body = format!(
        r#"{{
            "input": {{"text": "{}"}},
            "voice": {{"languageCode": "en-US"}},
            "audioConfig": {{
                "audioEncoding": "LINEAR16",
                "effectsProfileId": ["{}"]
            }}
        }}"#,
        text_to_speak, device_profile
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
    let audio_content = tts_response.bytes().await?;

    Ok(audio_content.to_vec())
}
