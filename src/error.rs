#[cfg(all(feature = "gpu", not(target_os = "macos")))]
use crate::cl;
use std::{error, fmt};

#[derive(Debug, Clone)]
/// Possible error states for the hashing.
pub enum Error {
    /// The allowed number of leaves cannot be greater than the arity of the tree.
    FullBuffer,
    /// Attempt to reference an index element that is out of bounds
    IndexOutOfBounds,
    /// The provided leaf was not found in the tree
    GPUError(String),
    #[cfg(all(feature = "gpu", not(target_os = "macos")))]
    ClError(cl::ClError),
    DecodingError,
    Other(String),
}

#[cfg(all(feature = "gpu", not(target_os = "macos")))]
impl From<cl::ClError> for Error {
    fn from(e: cl::ClError) -> Self {
        Self::ClError(e)
    }
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            Error::FullBuffer => write!(
                f,
                "The size of the buffer cannot be greater than the hash arity."
            ),
            Error::IndexOutOfBounds => write!(f, "The referenced index is outs of bounds."),
            Error::GPUError(s) => write!(f, "GPU Error: {}", s),
            #[cfg(all(feature = "gpu", not(target_os = "macos")))]
            Error::ClError(e) => write!(f, "OpenCL Error: {}", e),
            Error::DecodingError => write!(f, "PrimeFieldDecodingError"),
            Error::Other(s) => write!(f, "{}", s),
        }
    }
}
