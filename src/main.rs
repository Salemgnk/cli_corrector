mod ai;
mod commands;
mod config;

use clap::{Parser, Subcommand};
use std::io::{self, Write};
use std::process::Command;

#[derive(Parser)]
#[command(name = "cli_corrector")]
#[command(about = "A smart CLI corrector in Rust", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// The mistyped command to correct
    #[arg(trailing_var_arg = true)]
    mistyped: Vec<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Show correction statistics
    Stats,
    /// Manually add a correction
    Correct { mistyped: String, corrected: String },
    /// Print the shell integration script
    Init {
        #[arg(value_parser = ["zsh", "bash"])]
        shell: String,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let mut config = config::load_config();
    let mut history = config::load_history();

    match &cli.command {
        Some(Commands::Stats) => {
            println!("Stats feature coming soon...");
            return;
        }
        Some(Commands::Correct {
            mistyped,
            corrected,
        }) => {
            config
                .auto_correct
                .insert(mistyped.clone(), corrected.clone());
            config::save_config(&config);
            println!("Manual correction added: {} -> {}", mistyped, corrected);
            return;
        }
        Some(Commands::Init { shell }) => {
            if shell == "zsh" {
                println!(
                    r#"command_not_found_handler() {{
    cli_corrector "$@"
    return $?
}}"#
                );
            } else if shell == "bash" {
                println!(
                    r#"command_not_found_handle() {{
    cli_corrector "$@"
    return $?
}}"#
                );
            }
            return;
        }
        None => {
            if cli.mistyped.is_empty() {
                println!("Please provide a command to correct or run 'cli_corrector --help'");
                return;
            }
        }
    }

    // Reconstruction of the mistyped command and its arguments
    let cmd_name = &cli.mistyped[0];
    let cmd_args = if cli.mistyped.len() > 1 {
        cli.mistyped[1..].join(" ")
    } else {
        String::new()
    };

    // 1. Check auto-correct config
    if let Some(corrected) = config.auto_correct.get(cmd_name) {
        let full_cmd = if cmd_args.is_empty() {
            corrected.clone()
        } else {
            format!("{} {}", corrected, cmd_args)
        };
        println!("Auto-correcting '{}' -> '{}'", cmd_name, full_cmd);
        execute_command(&full_cmd);
        return;
    }

    let available = commands::load_available_commands();

    // 2. Try LLM first
    let mut suggestion = ai::suggest_command_llm(cmd_name, &available).await;

    // 3. Fallback to local fuzzy matching
    if suggestion.is_none() {
        suggestion = commands::suggest_command_local(cmd_name, &available);
    }

    if let Some(suggested_cmd) = suggestion {
        let full_suggested = if cmd_args.is_empty() {
            suggested_cmd.clone()
        } else {
            format!("{} {}", suggested_cmd, cmd_args)
        };

        print!("Did you mean: '{}' ? [y/N] ", full_suggested);
        let _ = io::stdout().flush();

        let mut input = String::new();
        // Dans certains contextes de shell hook, stdin n'est pas le terminal.
        // On essaie d'ouvrir /dev/tty pour forcer l'interaction, sinon on repli sur stdin.
        if let Ok(mut tty) = std::fs::File::open("/dev/tty") {
            use std::io::Read;
            let mut buf = [0; 1];
            while tty.read(&mut buf).unwrap_or(0) > 0 {
                let c = buf[0] as char;
                if c == '\n' || c == '\r' {
                    break;
                }
                input.push(c);
            }
            println!(); // saut de ligne après saisie
        } else {
            io::stdin().read_line(&mut input).unwrap_or(0);
        }

        if input.trim().eq_ignore_ascii_case("y") {
            execute_command(&full_suggested);

            let count = config::update_history(&mut history, cmd_name, &suggested_cmd);
            if count >= 3 && !config.auto_correct.contains_key(cmd_name) {
                print!(
                    "'{}' has been corrected to '{}' {} times. Enable auto-correction? [y/N] ",
                    cmd_name, suggested_cmd, count
                );
                let _ = io::stdout().flush();
                let mut auto_input = String::new();
                if let Ok(mut tty) = std::fs::File::open("/dev/tty") {
                    use std::io::Read;
                    let mut buf = [0; 1];
                    while tty.read(&mut buf).unwrap_or(0) > 0 {
                        let c = buf[0] as char;
                        if c == '\n' || c == '\r' {
                            break;
                        }
                        auto_input.push(c);
                    }
                    println!();
                } else {
                    io::stdin().read_line(&mut auto_input).unwrap_or(0);
                }
                if auto_input.trim().eq_ignore_ascii_case("y") {
                    config
                        .auto_correct
                        .insert(cmd_name.clone(), suggested_cmd.clone());
                    config::save_config(&config);
                    println!(
                        "Auto-correction enabled: '{}' -> '{}'",
                        cmd_name, suggested_cmd
                    );
                }
            }
        } else {
            println!("Command canceled.");
        }
    } else {
        println!("Unknown command: {}", cmd_name);
    }
}

fn execute_command(command: &str) {
    let mut child = Command::new("sh")
        .arg("-c")
        .arg(command)
        .spawn()
        .expect("Failed to execute command");

    let status = child.wait().expect("Failed to wait on child");

    if !status.success() {
        eprintln!("Command failed with status: {}", status);
    }
}
