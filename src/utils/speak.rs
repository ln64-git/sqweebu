// region: --- Modules
use std::process::Command;

use super::command::execute_command;
// endregion: --- Modules

pub fn speak(text: &str) -> () {
    execute_command(
        "aspeak",
        &["text", &format!("\"{}\"", text)],
        text.to_string(),
    );
}

pub fn speak_clipboard() {
    if let Ok(output) = Command::new("wl-paste").output() {
        let text = String::from_utf8_lossy(&output.stdout);
        // Define characters indicating end of sentence
        let sentence_endings = &['.', '!', '?'];
        // Split the text into sentences
        let mut sentence_index = 0;
        for (i, c) in text.char_indices() {
            if sentence_endings.contains(&c) {
                let sentence = &text[sentence_index..=i];
                speak(sentence.trim());
                sentence_index = i + 1; // Move the start to the next senten
            }
        }
        // Speak the remaining text if there's any
        if sentence_index < text.len() {
            let remaining_sentence = &text[sentence_index..];
            speak(remaining_sentence.trim());
        }
    }
}
