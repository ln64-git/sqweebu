// region: --- Modules
use super::command::execute_command;
use std::process::Command;
// endregion: --- Modules

fn toggle_dictation() {
    let process_check = Command::new("pgrep")
        .arg("-f")
        .arg("nerd-dictation begin")
        .output()
        .expect("Failed to execute pgrep");

    let is_running = process_check.status.success();
    let action_message = if is_running {
        "Dictation ended."
    } else {
        "Dictation started."
    };

    execute_command("aspeak", &["text"], action_message.to_owned());

    let command_to_execute = if is_running { "end" } else { "begin" };
    execute_command(
        "nerd-dictation",
        &[command_to_execute],
        format!("Nerd Dictation {}.", action_message),
    );
}

