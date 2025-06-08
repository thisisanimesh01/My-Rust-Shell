use std::io::{self, Write};
use std::process::{Command, Stdio};
use std::env;
use std::path::Path;

fn main() {
    loop {
        // Show prompt
        print!("rustsh> ");
        io::stdout().flush().unwrap();

        // Get user input
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            eprintln!("Failed to read input");
            continue;
        }

        // Trim and skip if empty
        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        // Split input into command and arguments
        let mut parts = input.split_whitespace();
        let command = parts.next().unwrap();
        let args: Vec<&str> = parts.collect();

        // Handle built-in commands
        match command {
            "cd" => {
                let new_dir = args.get(0).unwrap_or(&"/");
                let path = Path::new(new_dir);
                if let Err(e) = env::set_current_dir(path) {
                    eprintln!("cd error: {}", e);
                }
            }
            "exit" => {
                break;
            }
            _ => {
                // Run external command
                match Command::new(command)
                    .args(&args)
                    .stdin(Stdio::inherit())
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .spawn()
                {
                    Ok(mut child) => {
                        if let Err(e) = child.wait() {
                            eprintln!("Error waiting for command: {}", e);
                        }
                    }
                    Err(e) => {
                        eprintln!("Error executing '{}': {}", command, e);
                    }
                }
            }
        }
    }
}
