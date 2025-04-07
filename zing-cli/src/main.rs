use clap::{Parser, Subcommand};
use std::fs;
use std::io::{self, Read};
use zing_protocol::Command;

use zing::{Error, Result};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Optional subcommand (Play is default)
    #[command(subcommand)]
    command: Option<Commands>,

    /// Play a file or read from stdin if no file is provided
    #[arg(value_name = "FILE")]
    file: Option<String>,

    /// The time per chord (e.g., "500ms", "2s", "1m")
    #[arg(short, long, default_value = "100ms")]
    chord_duration: humantime::Duration,
}

#[derive(Subcommand)]
enum Commands {
    /// Plays a melody with the speaker, overrules any existing melodies
    Play {
        /// Play a file or read from stdin if no file is provided
        #[arg(value_name = "FILE")]
        file: Option<String>,

        /// The duration per chord (e.g., "500ms", "2s", "1m")
        #[arg(short, long, default_value = "100ms")]
        chord_duration: humantime::Duration,
    },

    /// Stop the currently playing melody
    Stop,

    /// Pauses the currently playing melody
    Pause,

    /// Resumes a paused melody
    Resume,
}

fn run() -> Result<()> {
    let cli = Cli::parse();

    let command = match cli.command {
        Some(cmd) => cmd,
        None => Commands::Play {
            file: cli.file,
            chord_duration: cli.chord_duration,
        },
    };

    match command {
        Commands::Play {
            file,
            chord_duration,
        } => {
            let notemap = if let Some(file) = file {
                fs::read_to_string(file).map_err(Error::Io)?
            } else {
                let mut buffer = String::new();
                io::stdin()
                    .read_to_string(&mut buffer)
                    .expect("Failed to read from stdin");
                buffer
            };

            let data = zing::chords_from_notemap(&notemap, &chord_duration)?;

            let play_data = zing_protocol::PlayData {
                chord_duration: *chord_duration,
                chords: data,
            };

            zing_protocol::send(&Command::Play(play_data)).map_err(Error::ZingProtocol)?;
        }

        Commands::Stop => zing_protocol::send(&Command::Stop).map_err(Error::ZingProtocol)?,
        Commands::Pause => zing_protocol::send(&Command::Pause).map_err(Error::ZingProtocol)?,
        Commands::Resume => zing_protocol::send(&Command::Resume).map_err(Error::ZingProtocol)?,
    }

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
