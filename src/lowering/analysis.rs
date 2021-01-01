use indexmap::IndexMap;

use crate::{
    lowering::{CompilationUnit, Diagnostics, Type, TypeId},
    syntax,
};
use std::collections::HashMap;

use super::{BitFields, Diagnostic, Field, LengthPrefixedString, Struct};

/// Analyse the `protocol.json` file's AST and convert it to the corresponding
/// Rust types.
pub fn lower(
    protocol: &crate::syntax::Protocol,
) -> Result<CompilationUnit, Diagnostics> {
    let mut analyser = Analyser::new();

    for (name, ty) in &protocol.types {
        let type_id = analyser.visit_type(ty);
        analyser.register_name(name, type_id);
    }

    analyser.finalise()
}

#[derive(Debug, Clone)]
struct Analyser {
    types: IndexMap<TypeId, Type>,
    named_types: IndexMap<String, TypeId>,
    last_id: TypeId,
    diagnostics: Diagnostics,
}

impl Analyser {
    fn new() -> Self {
        Analyser {
            types: IndexMap::new(),
            named_types: IndexMap::new(),
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

    fn lookup_by_name(&self, name: &str) -> Option<TypeId> {
        self.named_types.get(name).copied()
    }

    fn finalise(self) -> Result<CompilationUnit, Diagnostics> {
        let Analyser {
            types,
            named_types,
            diagnostics,
            ..
        } = self;

        if diagnostics.all_diagnostics().is_empty() {
            Ok(CompilationUnit { types, named_types })
        } else {
            Err(diagnostics)
        }
    }
}

impl Analyser {
    fn visit_type(&mut self, ty: &syntax::Type) -> TypeId {
        match ty {
            syntax::Type::Native => self.add_type(Type::Native),
            syntax::Type::Named(name) => match self.lookup_by_name(name) {
                Some(id) => id,
                None => {
                    self.diagnostics
                        .push(Diagnostic::MissingName { name: name.clone() });
                    TypeId::ERROR
                },
            },
            syntax::Type::Container(c) => self.visit_container(c),
            syntax::Type::Switch(s) => self.visit_switch(s),
            syntax::Type::BitFields(b) => self.visit_bitfields(b),
            syntax::Type::LengthPrefixedString { count_type } => {
                let count_type = self.visit_type(count_type);
                self.add_type(Type::LengthPrefixedString(
                    LengthPrefixedString { count_type },
                ))
            },
            syntax::Type::Mapper(m) => self.visit_mapper(m),
        }
    }

    fn visit_container(&mut self, container: &syntax::Container) -> TypeId {
        let mut fields = Vec::new();

        for field in &container.fields {
            match &field.ty {
                syntax::Type::Switch(_) => todo!("Handle switch fields"),
                other => {
                    let ty = self.visit_type(other);

                    match &field.name {
                        Some(name) => fields.push(Field {
                            name: name.clone(),
                            ty,
                        }),
                        None => todo!("Handle anonymous non-switch structs by flattening them"),
                    }
                },
            }
        }

        self.add_type(Type::Struct(Struct { fields }))
    }

    fn visit_switch(&mut self, _switch: &syntax::Switch) -> TypeId { todo!() }

    fn visit_bitfields(&mut self, bitfields: &syntax::BitFields) -> TypeId {
        self.add_type(Type::BitFields(BitFields {
            fields: bitfields.fields.clone(),
        }))
    }

    fn visit_mapper(&mut self, _mapper: &syntax::Mapper) -> TypeId { todo!() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn analyse_simple_struct() {
        let mut analyser = Analyser::new();
        let int = analyser.add_type(Type::Native);
        analyser.register_name("u32", int);

        let src = syntax::Container {
            fields: vec![
                syntax::Field {
                    name: Some("first".into()),
                    ty: syntax::Type::Named("u32".into()),
                },
                syntax::Field {
                    name: Some("second".into()),
                    ty: syntax::Type::Named("u32".into()),
                },
            ],
        };
        let should_be = Type::Struct(Struct {
            fields: vec![
                Field {
                    name: "first".into(),
                    ty: int,
                },
                Field {
                    name: "second".into(),
                    ty: int,
                },
            ],
        });

        let got = analyser.visit_container(&src);

        assert!(!got.is_error());
        assert_eq!(analyser.types[&got], should_be);
    }
}
