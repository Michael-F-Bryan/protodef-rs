use indexmap::IndexMap;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Protocol {
    pub types: IndexMap<String, Type>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Native,
    Named(String),
    Container(Container),
    Switch(Switch),
    BitFlags(BitFlags),
    Collection(Collection),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Container {
    pub fields: Vec<Field>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Field {
    pub name: String,
    pub ty: Type,
}

impl Field {
    pub fn new(name: impl Into<String>, ty: Type) -> Self {
        Field {
            name: name.into(),
            ty,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Switch;
#[derive(Debug, Clone, PartialEq)]
pub struct BitFlags;
#[derive(Debug, Clone, PartialEq)]
pub struct Collection;