#[derive(Debug, Clone, Default, PartialEq)]
pub struct Diagnostics(Vec<Diagnostic>);

impl Diagnostics {
    pub fn all_diagnostics(&self) -> &[Diagnostic] { &self.0 }

    pub(crate) fn push(&mut self, diag: Diagnostic) { self.0.push(diag); }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Diagnostic {
    MissingName { name: String },
}
