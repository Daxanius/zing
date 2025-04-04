use derive_more::{Display, Error, From};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Display, Error, From)]
pub enum Error {
    #[from]
    SerdeJson(serde_json::Error),

    #[from]
    InvalidCharacters(std::str::Utf8Error),

    #[from]
    Io(std::io::Error),
}
