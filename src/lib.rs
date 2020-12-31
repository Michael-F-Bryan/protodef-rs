//! A Rust code generator for the [ProtoDef][proto] protocol specification
//! format.
//!
//! # Examples
//!
//! ```rust,no_run
//! let src = include_str!("../tests/fixtures/protocol.json");
//!
//! let json = serde_json::from_str(src).unwrap();
//! let parsed = protodef::syntax::parse(&json).unwrap();
//! let analysed = protodef::lowering::lower(&parsed).unwrap();
//! let rust_code = protodef::backend::generate_rust(&analysed);
//!
//! println!("{}", rust_code);
//! ```
//!
//! [proto]: https://github.com/ProtoDef-io/ProtoDef

#[cfg(test)]
#[macro_use]
extern crate serde_json;
#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

pub mod backend;
pub mod lowering;
pub mod syntax;
