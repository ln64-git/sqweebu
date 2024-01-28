use std::process::Command;

pub fn execute_command(command: &str, args: &[&str], message: String) {
    match Command::new(command).args(args).output() {
        Ok(_) => println!("{}", message),
        Err(err) => eprintln!("Failed to execute {}: {}", command, err),
    }
}
