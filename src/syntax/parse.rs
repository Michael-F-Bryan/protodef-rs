use super::{
    errors::{Lookup, ResultExt, ValueExt, ValueKind},
    BitField, BitFields, Container, ErrorKind, Field, Mapper, ParseError,
    Protocol, Switch, Type,
};
use indexmap::IndexMap;
use serde_json::{Map, Value};

/// Parses a JSON document into a [`Protocol`].
pub fn parse(document: &Value) -> Result<Protocol, ParseError> {
    parse_document(document)
}

fn parse_document(document: &Value) -> Result<Protocol, ParseError> {
    let types = match document.get("types") {
        Some(types) => {
            let types = types.expect_object().with_context("types")?;
            parse_types(types).with_context("types")?
        },
        None => IndexMap::new(),
    };

    Ok(Protocol { types })
}

fn parse_types(
    types: &Map<String, Value>,
) -> Result<IndexMap<String, Type>, ParseError> {
    let mut parsed_types = IndexMap::new();

    for (name, ty) in types {
        let parsed = parse_type(ty).with_context(name)?;
        parsed_types.insert(name.clone(), parsed);
    }

    Ok(parsed_types)
}

fn parse_type(ty: &Value) -> Result<Type, ParseError> {
    if let Value::String(s) = ty {
        if s == "native" {
            return Ok(Type::Native);
        } else {
            return Ok(Type::Named(s.clone()));
        }
    }

    // if the type declaration isn't a string it'll be one of the fancy
    // "function" like declarations
    let array = ty.expect_array()?;
    expect_length(array, 2)?;

    let function_name =
        array[0].expect_string().with_context("function_name")?;
    let arg = &array[1];

    match function_name.as_str() {
        "container" => parse_container(arg)
            .map(Type::Container)
            .with_context("container"),
        "switch" => parse_switch(arg).map(Type::Switch).with_context("switch"),
        "bitfield" => parse_bitfields(arg)
            .map(Type::BitFields)
            .with_context("bitfield"),
        "pstring" => parse_length_prefixed_string(arg).with_context("pstring"),
        "mapper" => parse_mapper(arg).map(Type::Mapper).with_context("mapper"),

        "entityMetadataLoop" => {
            // TODO: Parse "entityMetadataLoop"
            Ok(Type::Native)
        },
        _ => Err(ParseError::unknown_function(function_name)),
    }
}

fn parse_container(arg: &Value) -> Result<Container, ParseError> {
    let raw_fields = arg.expect_array().with_context("fields")?;

    let mut fields = Vec::new();

    for (i, arg) in raw_fields.iter().enumerate() {
        let field = parse_field(arg).with_context("field").with_context(i)?;
        fields.push(field);
    }

    Ok(Container { fields })
}

fn parse_field(value: &Value) -> Result<Field, ParseError> {
    let value = value.expect_object()?;

    let ty = value.lookup("type")?;
    let ty = parse_type(ty).with_context("type")?;

    let name = if value.get("anon").is_some() {
        None
    } else {
        Some(value.lookup_string("name")?.clone())
    };

    Ok(Field { name, ty })
}

fn parse_switch(arg: &Value) -> Result<Switch, ParseError> {
    let args = arg.expect_object()?;

    let compare_to = args.lookup_string("compareTo")?.clone();

    let mut variants = IndexMap::new();

    for (key, value) in args.lookup_object("fields")? {
        let ty = parse_type(value).with_context("fields").with_context(key)?;
        let key = key
            .parse::<i64>()
            .map_err(|e| ParseError::new(ErrorKind::ParseInt(e)))?;

        variants.insert(key, ty);
    }

    let default = args
        .get("default")
        .map(parse_type)
        .transpose()?
        .map(Box::new);

    Ok(Switch {
        compare_to,
        variants,
        default,
    })
}

fn parse_length_prefixed_string(arg: &Value) -> Result<Type, ParseError> {
    let count_type = arg
        .expect_object()?
        .lookup("countType")
        .with_context("countType")?;

    let ty = parse_type(count_type).with_context("countType")?;

    Ok(Type::LengthPrefixedString {
        count_type: Box::new(ty),
    })
}

fn parse_bitfields(arg: &Value) -> Result<BitFields, ParseError> {
    let fields = arg.expect_array().with_context("fields")?;

    let mut bitfields = Vec::new();

    for (i, field) in fields.iter().enumerate() {
        let field = parse_bitfield(field)
            .with_context(i)
            .with_context("field")?;
        bitfields.push(field);
    }

    Ok(BitFields { fields: bitfields })
}

fn parse_bitfield(field: &Value) -> Result<BitField, ParseError> {
    let field = field.expect_object()?;

    let name = field.lookup_string("name")?.clone();

    let size = field.lookup_number("size")?;
    let size = size.as_i64().ok_or_else(|| {
        ParseError::new(ErrorKind::IncorrectType {
            expected: vec![ValueKind::Integer],
            found: ValueKind::for_number(size),
        })
    })? as usize;

    let signed = field.lookup_bool("signed")?;

    Ok(BitField { name, size, signed })
}

fn parse_mapper(_arg: &Value) -> Result<Mapper, ParseError> { todo!() }

#[track_caller]
fn expect_length<T>(items: &[T], expected: usize) -> Result<(), ParseError> {
    if items.len() == expected {
        Ok(())
    } else {
        Err(ParseError::new(ErrorKind::IncorrectArrayLength {
            expected,
            found: items.len(),
        }))
    }
}

#[cfg(test)]
mod tests {
    use crate::syntax::Switch;

    use super::*;

    #[test]
    fn parse_native() {
        let doc = json!("native");
        let should_be = Type::Native;

        let got = parse_type(&doc).unwrap();

        assert_eq!(got, should_be);
    }

    #[test]
    fn parse_named() {
        let doc = json!("u32");
        let should_be = Type::Named("u32".into());

        let got = parse_type(&doc).unwrap();

        assert_eq!(got, should_be);
    }

    #[test]
    fn parse_fields_for_simple_container() {
        let doc = json! {[
          "container",
          [
            {
              "name": "itemCount",
              "type": "i8"
            },
            {
              "name": "itemDamage",
              "type": "i16"
            },
            {
              "name": "nbtData",
              "type": "optionalNbt"
            }
          ]
        ]};
        let should_be = Type::Container(Container {
            fields: vec![
                Field::new("itemCount", Type::Named("i8".into())),
                Field::new("itemDamage", Type::Named("i16".into())),
                Field::new("nbtData", Type::Named("optionalNbt".into())),
            ]
            .into_iter()
            .collect(),
        });

        let got = parse_type(&doc).unwrap();

        assert_eq!(got, should_be);
    }

    #[test]
    #[ignore]
    fn parse_mapper() {
        let doc = json! {[
          "mapper",
          {
            "type": "varint",
            "mappings": {
                "0x00": "set_protocol",
                "0xfe": "legacy_server_list_ping"
            }
          }
        ]};
        let should_be = Type::Mapper(Mapper);

        let got = parse_type(&doc).unwrap();

        assert_eq!(got, should_be);
    }

    #[test]
    fn parse_switch() {
        let doc = json! {[
              "switch",
              {
                "compareTo": "blockId",
                "fields": {"-1": "void"},
                "default": ["container", []]
              }
        ]};
        let should_be = Type::Switch(Switch {
            compare_to: "blockId".into(),
            variants: vec![(-1, Type::Named("void".into()))]
                .into_iter()
                .collect(),
            default: Some(Box::new(Type::Container(Container {
                fields: Vec::new(),
            }))),
        });

        let got = parse_type(&doc).unwrap();

        assert_eq!(got, should_be);
    }

    #[test]
    fn parse_pstring() {
        let doc = json! {[
          "pstring",
          {
            "countType": "varint"
          }
        ]};
        let should_be = Type::LengthPrefixedString {
            count_type: Box::new(Type::Named("varint".into())),
        };

        let got = parse_type(&doc).unwrap();

        assert_eq!(got, should_be);
    }

    #[test]
    fn container_with_switch() {
        let doc = json! { [
          "container",
          [
            {
              "name": "name",
              "type": "varint",
            },
            {
              "name": "params",
              "type": [
                "switch",
                {
                  "compareTo": "name",
                  "fields": {}
                }
              ]
            }
          ]
        ]
        };
        let should_be = Type::Container(Container {
            fields: vec![
                Field::new("name", Type::Named("varint".into())),
                Field::new(
                    "params",
                    Type::Switch(Switch {
                        compare_to: "name".into(),
                        variants: IndexMap::new(),
                        default: None,
                    }),
                ),
            ],
        });

        let got = parse_type(&doc).unwrap();

        assert_eq!(got, should_be);
    }

    #[test]
    fn parse_bit_fields() {
        let doc = json! {[
          "bitfield",
          [
            {
              "name": "x",
              "size": 26,
              "signed": true
            },
            {
              "name": "y",
              "size": 12,
              "signed": true
            },
            {
              "name": "z",
              "size": 26,
              "signed": true
            }
          ]
        ]};
        let should_be = Type::BitFields(BitFields {
            fields: vec![
                BitField {
                    name: "x".into(),
                    size: 26,
                    signed: true,
                },
                BitField {
                    name: "y".into(),
                    size: 12,
                    signed: true,
                },
                BitField {
                    name: "z".into(),
                    size: 26,
                    signed: true,
                },
            ],
        });

        let got = parse_type(&doc).unwrap();

        assert_eq!(got, should_be);
    }

    #[test]
    fn parse_anon_field() {
        let doc = json! {{
          "anon": true,
          "type": [
            "bitfield",
            [
              {
                "name": "type",
                "size": 3,
                "signed": false
              }
            ]
          ]
        }};
        let should_be = Field {
            name: None,
            ty: Type::BitFields(BitFields {
                fields: vec![BitField {
                    name: "type".into(),
                    size: 3,
                    signed: false,
                }],
            }),
        };

        let got = parse_field(&doc).unwrap();

        assert_eq!(got, should_be);
    }
}
