use indexmap::IndexMap;
use std::fmt::{self, Debug, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub struct CompilationUnit {
    pub types: IndexMap<TypeId, Type>,
    pub named_types: IndexMap<String, TypeId>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Native,
    Struct(Struct),
    Enum(Enum),
    LengthPrefixedString(LengthPrefixedString),
    BitFields(BitFields),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Struct {
    pub fields: Vec<Field>,
}

/// A [`Struct`] field.
#[derive(Debug, Clone, PartialEq)]
pub struct Field {
    pub name: String,
    pub ty: TypeId,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Enum {
    pub compare_to: String,
    pub variants: Vec<Variant>,
}

/// A [`Enum`] variant.
#[derive(Debug, Clone, PartialEq)]
pub enum Variant {}

#[derive(Debug, Clone, PartialEq)]
pub struct BitFields {
    pub fields: Vec<crate::syntax::BitField>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LengthPrefixedString {
    pub count_type: TypeId,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct TypeId(u32);

impl TypeId {
    pub const ERROR: TypeId = TypeId(0);

    pub const fn is_error(self) -> bool {
        match self {
            TypeId::ERROR => true,
            _ => false,
        }
    }

    pub(crate) const fn next(self) -> Self { TypeId(self.0 + 1) }

    pub(crate) fn unique_name(self) -> String {
        format!("AnonymousType{}", self.0)
    }
}

impl Debug for TypeId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.is_error() {
            write!(f, "(error)")
        } else {
            write!(f, "TypeId({})", self.0)
        }
    }
}
