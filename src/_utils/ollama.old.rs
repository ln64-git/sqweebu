// Import necessary modules
use futures::StreamExt;
use ollama_rs::{generation::completion::request::GenerationRequest, Ollama};
use std::error::Error;
use tokio::sync::mpsc;

// Function to asynchronously generate text and send it to a channel
pub async fn generate_text(
    model: &str,
    final_prompt: String,
    tx: mpsc::Sender<String>,
) -> Result<(), Box<dyn Error>> {
    let ollama_instance = Ollama::default();
    let mut generation_stream = ollama_instance
        .generate_stream(GenerationRequest::new(model.to_string(), final_prompt))
        .await
        .map_err(|e| Box::new(e) as Box<dyn Error>)?;

    let mut sentence = String::new();
    while let Some(result) = generation_stream.next().await {
        let response_vec = result.map_err(|e| Box::new(e) as Box<dyn Error>)?;
        for generation_response in response_vec {
            let fragment = generation_response.response;
            sentence.push_str(&fragment);
            if fragment.ends_with(['.', '!', '?'].as_ref()) {
                if let Err(_) = tx.send(sentence.clone()).await {
                    // Handle the error or break if the receiver is dropped
                    break;
                }
                sentence.clear();
            }
        }
    }

    Ok(())
}
