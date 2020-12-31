#[derive(Debug, Clone, Default, PartialEq)]
pub struct Diagnostics(Vec<Diagnostic>);

#[derive(Debug, Clone, PartialEq)]
pub struct Diagnostic;
