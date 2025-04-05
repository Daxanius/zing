use derive_more::{Display, Error, From};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Display, Error, From)]
pub enum Error {
    #[from]
    Decode(bincode::error::DecodeError),

    #[from]
    Encode(bincode::error::EncodeError),

    #[from]
    Io(std::io::Error),
}
