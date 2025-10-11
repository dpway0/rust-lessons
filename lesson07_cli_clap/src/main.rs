use anyhow::{Context, Result};
use clap::{ArgAction, Parser, Subcommand};
use rand::{Rng, distr::Alphanumeric};
use std::{fs, path::PathBuf};

#[derive(Parser)]
#[command(name = "mycli", version, about = "Example CLI Tool")]
struct Cli {
    // Increase verbosity (-v, -vv, -vvv)
    #[arg(short, long, action = ArgAction::Count)]
    verbose: u8,

    // Length used when no subcommand is provided
    #[arg(short = 'n', long, default_value_t = 16)]
    len: usize,

    // If provided, ONLY read this file and print stats (contents + sizes)
    #[arg(short = 'f', long, value_name = "PATH")]
    file: Option<PathBuf>,

    // Subcommand is OPTIONAL (default = gen when no --file)
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    // Print greeting
    Hello {
        name: String,
    },

    // Add two numbers
    Add {
        a: i32,
        b: i32,
    },

    // Generate a random base62 ID
    Gen {
        // Length of the ID (overrides top-level --len)
        #[arg(short = 'n', long)]
        len: Option<usize>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    log(1, cli.verbose, "start");

    // If --file provided → ONLY do file stats and exit.
    if let Some(path) = &cli.file {
        log(2, cli.verbose, "file stats mode");
        let content = fs::read_to_string(path)
            .with_context(|| format!("failed to read {}", path.display()))?;
        println!("--- {} ---", path.display());
        println!("{content}");
        let bytes = content.len();
        let chars = content.chars().count();
        println!("bytes: {bytes}");
        println!("chars: {chars}");
        log(1, cli.verbose, "done");
        return Ok(());
    }

    // Otherwise: run subcommand (or default = gen)
    match &cli.command {
        Some(Commands::Hello { name }) => {
            log(2, cli.verbose, "hello");
            println!("Hi, {name}!");
        }
        Some(Commands::Add { a, b }) => {
            log(2, cli.verbose, "add");
            println!("{a} + {b} = {}", a + b);
        }
        Some(Commands::Gen { len }) => {
            log(2, cli.verbose, "gen");
            let use_len = len.unwrap_or(cli.len);
            println!("{}", gen_id(use_len));
        }
        None => {
            // default = gen
            log(2, cli.verbose, "gen (default)");
            println!("{}", gen_id(cli.len));
        }
    }

    log(1, cli.verbose, "done");
    Ok(())
}

fn gen_id(len: usize) -> String {
    rand::rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}

// Log to stderr if `verbose` level is high enough
fn log(level: u8, verbose: u8, msg: &str) {
    if verbose >= level {
        eprintln!("[v{level}] {msg}");
    }
}
