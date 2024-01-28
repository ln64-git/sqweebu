// region: --- Modules
use crate::speak::speak;
use ollama_rs::generation::completion::request::GenerationRequest;
use ollama_rs::Ollama;
// endregion: --- Modules

pub async fn generate_text(model: &str, final_prompt: String) {
    let ollama_instance = Ollama::default();
    let mut generation_stream = ollama_instance
        .generate_stream(GenerationRequest::new(model.to_string(), final_prompt))
        .await
        .unwrap();

    let mut sentence = String::new();
    let mut sentence_init = true;
    while let Some(result) = futures::stream::StreamExt::next(&mut generation_stream).await {
        let result = result.unwrap();
        for generation_response in result {
            let fragment = generation_response.response;
            sentence.push_str(&fragment);
            if fragment.contains('.') || fragment.contains('!') || fragment.contains('?') {
                if !sentence_init {
                    sentence = sentence.chars().skip(1).collect();
                }
                speak(sentence.as_str());
                sentence.clear();
                sentence_init = false;
            }
        }
    }
}
