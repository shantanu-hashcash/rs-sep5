#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Invalid index provided for path {path}")]
    InvalidIndex { path: String },

    #[error(transparent)]
    Bip32(#[from] bip39::ErrorKind),

    #[error("Unknown error from bip32")]
    Unknown,
}
