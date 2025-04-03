use zing_protocol::Command;
use clap::{ Parser, Subcommand };
use std::fs;
use std::io::{ self, Read };

use zing_cli::{ Error, Result };

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Play a file or read from stdin if no file is provided
    #[arg(value_name = "FILE")]
    file: Option<String>,
        
    /// The time per chord (e.g., "500ms", "2s", "1m")
    #[arg(short, long, default_value = "100ms")]
    chord_duration: humantime::Duration,
 
    /// Optional subcommand (Play is default)
    #[command(subcommand)]
    command: Option<Commands>,
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
    Resume
}

fn run() -> Result<()> {
    let cli = Cli::parse();

    let command = match cli.command {
        Some(cmd) => cmd,
        None => Commands::Play { file: cli.file, chord_duration: cli.chord_duration },
    };

    match command {
        Commands::Play{ file, chord_duration } => {
            let notemap = match file { 
                Some(file) => fs::read_to_string(file).map_err(|e| Error::Io(e))?,
                None => {
                    let mut buffer = String::new();
                    io::stdin().read_to_string(&mut buffer).expect("Failed to read from stdin");
                    buffer
                }
            };
            
            let data = zing_cli::chords_from_notemap(&notemap, &chord_duration)?;

            let play_data = zing_protocol::PlayData {
                chord_duration: *chord_duration,
                chords: data
            };

            zing_protocol::send(Command::Play(play_data)).map_err(|e| Error::ZingProtocol(e))?;
        },

        Commands::Stop => zing_protocol::send(Command::Stop).map_err(|e| Error::ZingProtocol(e))?,
        Commands::Pause => zing_protocol::send(Command::Pause).map_err(|e| Error::ZingProtocol(e))?,
        Commands::Resume => zing_protocol::send(Command::Resume).map_err(|e| Error::ZingProtocol(e))?,
    }

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

