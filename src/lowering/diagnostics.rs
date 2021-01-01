#[derive(Debug, Clone, Default, PartialEq)]
pub struct Diagnostics(Vec<Diagnostic>);

impl Diagnostics {
    pub fn all_diagnostics(&self) -> &[Diagnostic] { &self.0 }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Diagnostic;
