//! Core abstractions and types used by ProtoDef-generated code.

pub mod native;

use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

/// Deserialize something from its binary form.
pub trait Deserialize<'de>: Sized {
    fn deserialize(
        buffer: &'de [u8],
    ) -> Result<(Self, &'de [u8]), DeserializeError>;
}

/// Any errors that can occur in [`Deserialize::deserialize()`].
#[derive(Debug)]
pub enum DeserializeError {
    Custom(Box<dyn Error>),
}

impl Display for DeserializeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Custom error")
    }
}

impl Error for DeserializeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            DeserializeError::Custom(inner) => Some(&**inner),
        }
    }
}
