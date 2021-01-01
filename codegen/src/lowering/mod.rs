//! Lowering from the `protocol.json` AST to their equivalent Rust types.

mod analysis;
mod diagnostics;
mod hir;

pub use analysis::lower;
pub use diagnostics::{Diagnostic, Diagnostics};
pub use hir::*;
