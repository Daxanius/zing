use derive_more::{ Display, Error };

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Display, Error)]
pub enum Error {
    ZingProtocol(zing_protocol::Error),
    Io(std::io::Error),
    Beep(beep::Error),
    Convert,
    LockPoisoned,
    CouldNotJoinThread, 
    NoChordsProvided,
}
