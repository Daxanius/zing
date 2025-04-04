use std::fmt;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    ZingProtocol(zing_protocol::Error),
    Io(std::io::Error),
    NoteDoesNotExist(char),
    OctaveDoesNotExist(usize),
    InvalidOctave(String),
    OctaveNotSpecified,
    NotesNotSpecified,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::ZingProtocol(e) => write!(f, "Could not communicate with daemon: {e}"),
            Error::Io(e) => write!(f, "Io error: {e}"),
            Error::NoteDoesNotExist(c) => write!(f, "Note does dot exist: '{c}'"),
            Error::OctaveDoesNotExist(o) => write!(f, "Octave does not exist: {o}"),
            Error::InvalidOctave(e) => write!(f, "Invalid octave: {e}"),
            Error::OctaveNotSpecified => write!(f, "No octave specified"),
            Error::NotesNotSpecified => write!(f, "No notes specified"),
        }
    }
}

impl std::error::Error for Error {}
