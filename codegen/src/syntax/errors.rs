use std::{
    error::Error,
    fmt::{self, Display, Formatter},
    panic::Location,
};

use serde_json::{map::Map, Number, Value};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ValueKind {
    Object,
    Array,
    String,
    Integer,
    Float,
    Bool,
}

impl ValueKind {
    pub(crate) fn for_number(n: Number) -> Self {
        if n.is_i64() || n.is_u64() {
            ValueKind::Integer
        } else {
            ValueKind::Float
        }
    }
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

impl Error for ParseError {}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if !self.context.is_empty() {
            let breadcrumbs = self.context.join(" > ");
            write!(f, "At \"{}\" ", breadcrumbs)?;
        }

        match &self.kind {
            ErrorKind::IncorrectType { expected, found } => {
                write!(f, "incorrect type, expected ")?;

                if expected.len() == 1 {
                    write!(f, "{:?}", expected[0])?;
                } else {
                    let possibilities = expected
                        .iter()
                        .map(|v| format!("{:?}", v))
                        .collect::<Vec<_>>()
                        .join(", ");
                    write!(f, "one of {}", possibilities)?;
                }

                write!(f, " but found {:?}", found)?;

                Ok(())
            },
            ErrorKind::UnknownFunction { name } => {
                write!(f, "unable to handle a \"{}\"", name)
            },
            ErrorKind::MissingField { name } => {
                write!(f, "missing the \"{}\" field", name)
            },
            ErrorKind::ParseInt(_) => {
                write!(f, "unable to parse the value as an integer")
            },
            ErrorKind::IncorrectArrayLength { expected, found } => write!(
                f,
                "incorrect array length, expected {} but found {}",
                expected, found
            ),
        }
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
    IncorrectArrayLength {
        expected: usize,
        found: usize,
    },
}

pub(crate) trait ResultExt<T> {
    fn with_context(self, context: impl Display) -> Result<T, ParseError>;
}

impl<T> ResultExt<T> for Result<T, ParseError> {
    fn with_context(self, context: impl Display) -> Result<T, ParseError> {
        self.map_err(|mut e| {
            e.context.insert(0, context.to_string());
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
    #[track_caller]
    fn expect_number(&self) -> Result<Number, ParseError>;
    #[track_caller]
    fn expect_bool(&self) -> Result<bool, ParseError>;
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

    fn expect_number(&self) -> Result<Number, ParseError> {
        match self {
            Value::Number(n) => Ok(n.clone()),
            other => Err(ParseError::new(ErrorKind::IncorrectType {
                expected: vec![ValueKind::Integer, ValueKind::Float],
                found: other.value_kind(),
            })),
        }
    }

    fn expect_bool(&self) -> Result<bool, ParseError> {
        match self {
            Value::Bool(b) => Ok(*b),
            other => Err(ParseError::incorrect_type(
                ValueKind::Bool,
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
        match self {
            Some(value) => Ok(value),
            None => Err(ParseError::missing_field(name)),
        }
    }
}

pub(crate) trait Lookup<K> {
    #[track_caller]
    fn lookup(&self, key: K) -> Result<&Value, ParseError>;

    #[track_caller]
    fn lookup_string(&self, key: K) -> Result<&String, ParseError>
    where
        K: Copy + Display,
    {
        self.lookup(key)?.expect_string().with_context(key)
    }

    #[track_caller]
    fn lookup_object(&self, key: K) -> Result<&Map<String, Value>, ParseError>
    where
        K: Copy + Display,
    {
        self.lookup(key)?.expect_object().with_context(key)
    }

    #[track_caller]
    fn lookup_number(&self, key: K) -> Result<Number, ParseError>
    where
        K: Copy + Display,
    {
        self.lookup(key)?.expect_number().with_context(key)
    }

    #[track_caller]
    fn lookup_bool(&self, key: K) -> Result<bool, ParseError>
    where
        K: Copy + Display,
    {
        self.lookup(key)?.expect_bool().with_context(key)
    }
}

impl<'a> Lookup<&'a str> for Map<String, Value> {
    fn lookup(&self, key: &'a str) -> Result<&Value, ParseError> {
        self.get(key).or_missing_field(key)
    }
}

impl Lookup<usize> for Vec<Value> {
    fn lookup(&self, key: usize) -> Result<&Value, ParseError> {
        self.get(key).or_missing_field(key)
    }
}
