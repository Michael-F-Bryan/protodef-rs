use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Diagnostics(Vec<Diagnostic>);

impl Diagnostics {
    pub fn all_diagnostics(&self) -> &[Diagnostic] { &self.0 }

    pub(crate) fn push(&mut self, diag: Diagnostic) { self.0.push(diag); }
}

impl Error for Diagnostics {}

impl Display for Diagnostics {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.0.len() {
            0 => write!(f, "no issues"),
            1 => write!(f, "{}", &self.0[0]),
            len => {
                writeln!(f, "{} issues found:", len)?;

                for diagnostic in &self.0 {
                    writeln!(f, "  {}", diagnostic)?;
                }

                Ok(())
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Diagnostic {
    MissingName { name: String },
}

impl Display for Diagnostic {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Diagnostic::MissingName { name } => {
                writeln!(f, "missing name: {}", name)
            },
        }
    }
}
