//! Code for working with the `protocol.json` AST.
//!
//! Your main entrypoint into the AST will be via the [`parse()`] function.
//!
//! ```rust
//! let document = serde_json::json!({
//!   "types": {
//!     "varint": "native",
//!     "i64": "native",
//!     "position": [
//!       "bitfield",
//!       [
//!         { "name": "x", "size": 26, "signed": true },
//!         { "name": "y", "size": 12, "signed": true },
//!         { "name": "z", "size": 26, "signed": true }
//!       ]
//!     ],
//!     "orientation": [
//!       "container",
//!       [
//!           { "name": "pitch", "type": "f32" },
//!           { "name": "yaw", "type": "f32" },
//!           { "name": "roll", "type": "f32" }
//!       ]
//!     ]
//!   }
//! });
//!
//! let protocol = protodef::syntax::parse(&document)?;
//!
//! assert!(protocol.types.contains_key("i64"));
//! assert!(protocol.types.contains_key("orientation"));
//! # Result::<(), protodef::syntax::ParseError>::Ok(())
//! ```

mod errors;
mod parse;
mod types;

pub use errors::{ErrorKind, ParseError};
pub use parse::parse;
pub use types::*;
