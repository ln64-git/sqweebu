// region: --- Modules
mod utils;
use crate::clipboard::clipboard;
use crate::speak::speak_clipboard;
use crate::utils::clipboard;
use std::env;
use utils::speak::speak;
use utils::{ollama, speak};
// endregion: --- Modules

#[tokio::main]
async fn main() {
    // speak("Chatbot initialized.");
    let model = "llama2-uncensored";
    let binding = "--help".to_string();
    let args: Vec<String> = env::args().collect();
    let primary_function = args.get(1).unwrap_or(&binding);
    match primary_function.as_str() {
        "--converse" => {
            speak("Do you have something you'd like to say?");
            return;
        }
        "--pull_clipboard" => {
            let secondary_function = args.get(2).unwrap_or(&binding);
            match secondary_function.as_str() {
                "--converse" => {
                    speak("What is it?");
                }
                "--speak" => {
                    speak_clipboard();
                }
                "--respond" => {
                    let default_prompt_prelude = "Explain this...";
                    let prompt_prelude = args
                        .get(3)
                        .map(|s| s.as_str())
                        .unwrap_or(default_prompt_prelude);
                    let prompt_input = match clipboard::clipboard() {
                        Ok(text) => text,
                        Err(err) => {
                            eprintln!("Error: Unable to paste text from the clipboard: {}", err);
                            return;
                        }
                    };
                    let final_prompt = format!("{} {}", prompt_prelude, prompt_input);
                    ollama::generate_text(model, final_prompt).await;
                }
                _ => {
                    return;
                }
            }
        }
        "--help" => {
            println!("Usage: chatbot [--converse | --parse_clipboard [--speak_clipboard | --response] [prompt_prelude]]");
        }
        _ => {
            return;
        }
    }
}
