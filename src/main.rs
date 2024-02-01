use dotenv::dotenv;
use futures::Stream;
use response_engine::{generate_text, get_azure_response, get_clipboard, listen_to_audio_stream};
use std::env;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let subscription_key = env::var("API_KEY")?;
    let region = "eastus";
    let voice_gender = "Female";
    let voice_name = "en-US-JennyNeural";
    let output_format = "audio-48khz-192kbitrate-mono-mp3";

    let prompt_final = format!("{}{}", "Explain this...", "Rome");
    println!("{:?}\n", prompt_final);

    let model = "llama2-uncensored";

    // Check if generate_text returns an error.
    let sentences = generate_text(model, prompt_final).await?;
    for sentence in sentences {
        println!("{}", sentence);
        let tts_response = get_azure_response(
            &subscription_key,
            &region,
            &sentence,
            &voice_gender,
            &voice_name,
            &output_format,
        )
        .await?; // Handle the Result from get_azure_response
        listen_to_audio_stream(tts_response).await?; // Handle the Result from listen_to_audio_stream
    }

    Ok(())
}
