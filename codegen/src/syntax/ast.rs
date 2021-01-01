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
    BitFields(BitFields),
    LengthPrefixedString { count_type: Box<Type> },
    Mapper(Mapper),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Container {
    pub fields: Vec<Field>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Field {
    pub name: Option<String>,
    pub ty: Type,
}

impl Field {
    pub fn new(name: impl Into<String>, ty: Type) -> Self {
        Field {
            name: Some(name.into()),
            ty,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Switch {
    pub compare_to: String,
    pub variants: IndexMap<i64, Type>,
    pub default: Option<Box<Type>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BitFields {
    pub fields: Vec<BitField>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BitField {
    pub name: String,
    pub size: usize,
    pub signed: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Mapper;
