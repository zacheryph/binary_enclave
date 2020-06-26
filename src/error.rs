use thiserror::Error;

/// Result for Enclave operations
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Error that can be produced reading/writing to the Enclave
#[derive(Debug, Error)]
pub enum Error {
    /// Attempted to write a payload larger than the Enclave size.
    #[error("Payload exceeds section size. {} > {}.", payload, section)]
    SectionSizeExceeded { payload: usize, section: usize },

    /// Binary section not located. missing `#[enclave]`?
    #[error("Section not found")]
    SectionNotFound(String),

    /// Error occured during Enclave deserialization. Binary tampering?
    #[error("Payload decoding error")]
    PayloadDecoding(#[from] Box<bincode::ErrorKind>),

    /// Failed to interpret binary. Simply put, this should never happen.
    #[error("Binary decoding error")]
    BinaryDecoding(#[from] goblin::error::Error),

    /// Something failed during File IO operations.
    #[error("File Handling Error")]
    File(#[from] std::io::Error),

    /// The best kind of errors.
    #[error("Unknown Error")]
    Other,
}
