use bincode::{
    Decode, Encode,
    config::{self, Configuration},
};
use std::io::Write;
use std::os::unix::net::UnixStream;
use std::time::Duration;

pub use error::{Error, Result};

pub mod error;

pub const SOCKET_PATH: &str = "/run/zingd.sock";

#[derive(Encode, Decode, Debug, Default, Clone)]
pub struct Chord {
    pub extended_duration: Duration,
    pub notes: Vec<u16>,
}

#[derive(Encode, Decode, Debug, Default)]
pub struct PlayData {
    pub chord_duration: Duration,
    pub chords: Vec<Chord>,
}

#[derive(Encode, Decode, Debug)]
pub enum Command {
    Play(PlayData),
    Stop,
    Pause,
    Resume,
}

impl Command {
    /// Deserializes a `Command` from a byte sequence.
    ///
    /// Attempts to convert the given byte slice into a valid `Command` instance.
    ///
    /// # Errors
    /// Returns an error if:
    /// - The sequence cannot be deserialized
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        Ok(bincode::decode_from_slice(bytes, Self::get_config())
            .map_err(Error::Decode)?
            .0)
    }

    /// Serializes the `Command` into a byte sequence
    ///
    /// This is typically used for sending commands over a socket or writing to a file.
    ///
    /// # Errors
    /// Returns an error if:
    /// - The command cannot be serialized.
    pub fn as_bytes(&self) -> Result<Vec<u8>> {
        bincode::encode_to_vec(self, Self::get_config()).map_err(Error::Encode)
    }

    fn get_config() -> Configuration {
        config::standard()
    }
}

/// Sends a serialized command to the Unix socket daemon.
///
/// This function connects to the Unix socket specified by `SOCKET_PATH`
/// and writes the given command as a byte stream.
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
