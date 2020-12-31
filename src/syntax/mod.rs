//! Code for working with the `protocol.json` AST.

mod errors;
mod parse;
mod types;

pub use errors::{ErrorKind, ParseError};
pub use parse::parse;
pub use types::{
    BitFlags, Collection, Container, Field, Protocol, Switch, Type,
};
