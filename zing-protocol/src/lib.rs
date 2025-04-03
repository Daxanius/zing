use serde::{ Serialize, Deserialize };
use std::os::unix::net::UnixStream;
use std::time::Duration;
use std::str::from_utf8;
use std::io::Write;

pub use error::{ Error, Result };

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
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let s = from_utf8(bytes).map_err(|e| Error::InvalidCharacters(e))?;
        serde_json::from_str(s).map_err(|e| Error::SerdeJson(e)) 
    }

    pub fn as_bytes(&self) -> Result<Vec<u8>> {
        let s = serde_json::to_string(self).map_err(|e| Error::SerdeJson(e))?;
        Ok(s.into_bytes())
    }
}

pub fn send(command: Command) -> Result<()> {
    match UnixStream::connect(SOCKET_PATH) {
        Ok(mut stream) => {
            let message = command.as_bytes()?;
            let _ = stream.write_all(&message);
            Ok(())
        },
        Err(e) => Err(Error::Io(e))
    }
}


