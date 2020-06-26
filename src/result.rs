use thiserror::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Payload exceeds section size. {} > {}.", payload, section)]
    SectionSizeExceeded { payload: usize, section: usize },

    #[error("Section not found")]
    SectionNotFound(String),

    #[error("Payload decoding error")]
    PayloadDecoding(#[from] Box<bincode::ErrorKind>),

    #[error("Binary decoding error")]
    BinaryDecoding(#[from] goblin::error::Error),

    #[error("File Handling Error")]
    File(#[from] std::io::Error),

    #[error("Unknown Error")]
    Other,
}
