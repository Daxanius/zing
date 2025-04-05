use serde::{Deserialize, Serialize};
use std::io::Write;
use std::os::unix::net::UnixStream;
use std::str::from_utf8;
use std::time::Duration;

pub use error::{Error, Result};

pub mod error;

pub const SOCKET_PATH: &str = "/run/zingd.sock";

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Chord {
    pub extended_duration: Duration,
    pub notes: Vec<u16>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PlayData {
    pub chord_duration: Duration,
    pub chords: Vec<Chord>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Command {
    Play(PlayData),
    Stop,
    Pause,
    Resume,
}

impl Command {
    /// Deserializes a `Command` from a UTF-8 byte sequence.
    ///
    /// Attempts to convert the given byte slice into a valid `Command` instance.
    /// The bytes must represent a UTF-8 encoded JSON string matching the structure of `Command`.
    ///
    /// # Arguments
    /// * `bytes` - A byte slice containing the serialized command data.
    ///
    /// # Errors
    /// Returns an error if:
    /// - The byte slice contains invalid UTF-8 characters.
    /// - The JSON structure is malformed or does not match the expected format.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let s = from_utf8(bytes).map_err(Error::InvalidCharacters)?;
        serde_json::from_str(s).map_err(Error::SerdeJson)
    }

    /// Serializes the `Command` into a UTF-8 encoded JSON byte sequence.
    ///
    /// Converts the `Command` instance into a JSON string, then encodes it as a byte vector.
    /// This is typically used for sending commands over a socket or writing to a file.
    ///
    /// # Errors
    /// Returns an error if:
    /// - The command cannot be serialized into JSON due to a serialization failure.
    pub fn as_bytes(&self) -> Result<Vec<u8>> {
        let s = serde_json::to_string(self).map_err(Error::SerdeJson)?;
        Ok(s.into_bytes())
    }
}

/// Sends a serialized command to the Unix socket daemon.
///
/// This function connects to the Unix socket specified by `SOCKET_PATH`
/// and writes the given command as a byte stream.
///
/// # Arguments
/// * `command` - The command to serialize and send to the daemon.
///
/// # Errors
/// Returns an error if:
/// - The Unix socket at `SOCKET_PATH` does not exist or cannot be connected to.
/// - The command serialization (`command.as_bytes()`) fails.
/// - Writing to the socket fails.
///
/// # Side effects
/// - Performs I/O over a Unix socket.
pub fn send(command: &Command) -> Result<()> {
    match UnixStream::connect(SOCKET_PATH) {
        Ok(mut stream) => {
            let message = command.as_bytes()?;
            let _ = stream.write_all(&message);
            Ok(())
        }
        Err(e) => Err(Error::Io(e)),
    }
}
