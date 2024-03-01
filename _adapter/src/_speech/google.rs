use dotenv::dotenv;
use std::env;
use std::error::Error;

pub async fn get_google_audio_response(text_to_speak: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    dotenv().ok();
    let api_key =
        env::var("GOOGLE_API_KEY").expect("GOOGLE_API_KEY not found in environment variables");

    let endpoint = "https://texttospeech.googleapis.com/v1/text:synthesize";
    let output_audio_config = r#"{"audioEncoding":"MP3"}"#;

    let tts_response = reqwest::Client::new()
        .post(endpoint)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .body(format!(
            r#"{{
                "input":{{
                    "text":"{}"
                }},
                "voice":{{
                    "languageCode":"en-US",
                    "name":"en-US-Wavenet-F"
                }},
                "audioConfig":{}
            }}"#,
            text_to_speak, output_audio_config
        ))
        .send()
        .await?;

    // Extract audio content
    let audio_content = tts_response.bytes().await?;
    Ok(audio_content.into_iter().collect())
}
