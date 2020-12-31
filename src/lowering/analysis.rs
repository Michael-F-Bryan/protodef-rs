use std::collections::HashMap;

use super::{CompilationUnit, Diagnostics, Type, TypeId};

/// Analyse the `protocol.json` file's AST and convert it to the corresponding
/// Rust types.
pub fn analyse(
    _protocol: &crate::syntax::Protocol,
) -> Result<CompilationUnit, Diagnostics> {
    let mut analyser = Analyser::new();
    todo!()
}

#[derive(Debug, Clone)]
struct Analyser {
    types: HashMap<TypeId, Type>,
    named_types: HashMap<String, TypeId>,
    last_id: TypeId,
    diagnostics: Diagnostics,
}

impl Analyser {
    fn new() -> Self {
        Analyser {
            types: HashMap::new(),
            named_types: HashMap::new(),
            last_id: TypeId::ERROR,
            diagnostics: Diagnostics::default(),
        }
    }

    fn add_type(&mut self, ty: Type) -> TypeId {
        let id = self.last_id.next();
        self.last_id = id;

        self.types.insert(id, ty);

        id
    }

    fn register_name(&mut self, name: impl Into<String>, type_id: TypeId) {
        self.named_types.insert(name.into(), type_id);
    }
}
