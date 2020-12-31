use std::{fmt::Display, panic::Location};

use serde_json::{map::Map, Value};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ValueKind {
    Object,
    Array,
    String,
}

#[derive(Debug, Clone)]
pub struct ParseError {
    pub context: Vec<String>,
    pub kind: ErrorKind,
    pub location: &'static Location<'static>,
}

impl ParseError {
    #[track_caller]
    pub(crate) fn new(kind: ErrorKind) -> Self {
        ParseError {
            context: Vec::new(),
            kind,
            location: Location::caller(),
        }
    }

    #[track_caller]
    pub(crate) fn incorrect_type(
        expected: ValueKind,
        found: ValueKind,
    ) -> Self {
        ParseError::new(ErrorKind::IncorrectType {
            expected: vec![expected],
            found,
        })
    }

    #[track_caller]
    pub(crate) fn unknown_function(name: impl Into<String>) -> Self {
        ParseError::new(ErrorKind::UnknownFunction { name: name.into() })
    }

    #[track_caller]
    pub(crate) fn missing_field(name: impl Display) -> Self {
        ParseError::new(ErrorKind::MissingField {
            name: name.to_string(),
        })
    }
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum ErrorKind {
    IncorrectType {
        expected: Vec<ValueKind>,
        found: ValueKind,
    },
    UnknownFunction {
        name: String,
    },
    MissingField {
        name: String,
    },
    ParseInt(std::num::ParseIntError),
}

pub(crate) trait ResultExt<T> {
    fn with_context(self, context: impl Into<String>) -> Result<T, ParseError>;
}

impl<T> ResultExt<T> for Result<T, ParseError> {
    fn with_context(self, context: impl Into<String>) -> Result<T, ParseError> {
        self.map_err(|mut e| {
            e.context.insert(0, context.into());
            e
        })
    }
}

pub(crate) trait ValueExt {
    fn value_kind(&self) -> ValueKind;

    #[track_caller]
    fn expect_object(&self) -> Result<&Map<String, Value>, ParseError>;
    #[track_caller]
    fn expect_array(&self) -> Result<&Vec<Value>, ParseError>;
    #[track_caller]
    fn expect_string(&self) -> Result<&String, ParseError>;
}

impl ValueExt for Value {
    fn expect_object(&self) -> Result<&Map<String, Value>, ParseError> {
        match self {
            Value::Object(map) => Ok(map),
            other => Err(ParseError::incorrect_type(
                ValueKind::Object,
                other.value_kind(),
            )),
        }
    }

    fn expect_array(&self) -> Result<&Vec<Value>, ParseError> {
        match self {
            Value::Array(arr) => Ok(arr),
            other => Err(ParseError::incorrect_type(
                ValueKind::Array,
                other.value_kind(),
            )),
        }
    }

    fn expect_string(&self) -> Result<&String, ParseError> {
        match self {
            Value::String(s) => Ok(s),
            other => Err(ParseError::incorrect_type(
                ValueKind::String,
                other.value_kind(),
            )),
        }
    }

    fn value_kind(&self) -> ValueKind {
        match self {
            Value::Object(_) => ValueKind::Object,
            Value::String(_) => ValueKind::String,
            Value::Array(_) => ValueKind::Array,
            _ => todo!(),
        }
    }
}

pub(crate) trait OptionExt<T> {
    #[track_caller]
    fn or_missing_field(self, name: impl Display) -> Result<T, ParseError>;
}

impl<T> OptionExt<T> for Option<T> {
    fn or_missing_field(self, name: impl Display) -> Result<T, ParseError> {
        self.ok_or_else(|| ParseError::missing_field(name))
    }
}

pub(crate) trait Lookup<T, K> {
    #[track_caller]
    fn lookup(&self, key: K) -> Result<&T, ParseError>;
}

impl<'a> Lookup<Value, &'a str> for Map<String, Value> {
    fn lookup(&self, key: &'a str) -> Result<&Value, ParseError> {
        self.get(key).or_missing_field(key)
    }
}

impl Lookup<Value, usize> for Vec<Value> {
    fn lookup(&self, key: usize) -> Result<&Value, ParseError> {
        self.get(key).or_missing_field(key)
    }
}
