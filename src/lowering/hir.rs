use std::{
    collections::HashMap,
    fmt::{self, Debug, Formatter},
};

#[derive(Debug, Clone, PartialEq)]
pub struct CompilationUnit {
    pub types: HashMap<TypeId, Type>,
    pub named_types: HashMap<String, TypeId>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Native,
    Struct(Struct),
    Enum(Enum),
    LengthPrefixedString(LengthPrefixedString),
    BitFields(BitFields),
    Collection(Collection),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Struct;

#[derive(Debug, Clone, PartialEq)]
pub struct Enum;

#[derive(Debug, Clone, PartialEq)]
pub struct BitFields;

#[derive(Debug, Clone, PartialEq)]
pub struct LengthPrefixedString {
    pub count_type: TypeId,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Collection {
    pub item_type: TypeId,
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
