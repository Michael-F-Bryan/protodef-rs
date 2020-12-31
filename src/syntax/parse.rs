use super::{
    errors::{Lookup, ResultExt, ValueExt},
    Container, ErrorKind, Field, ParseError, Protocol, Switch, Type,
};
use indexmap::IndexMap;
use serde_json::{Map, Value};

pub fn parse(document: Value) -> Result<Protocol, ParseError> {
    parse_document(document)
}

fn parse_document(document: Value) -> Result<Protocol, ParseError> {
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
    if array.is_empty() {
        todo!()
    }

    let (function_name, args) = array.split_first().expect("Already checked");
    let function_name = function_name
        .expect_string()
        .with_context("function name")?;

    match function_name.as_str() {
        "container" => parse_container(args)
            .map(Type::Container)
            .with_context("container"),

        "switch" => parse_switch(args).map(Type::Switch).with_context("switch"),

        "bitflags" => todo!(),
        "pstring" => parse_length_prefixed_string(args).with_context("pstring"),
        "collection" => todo!(),
        "mapping" => todo!(),

        _ => Err(ParseError::unknown_function(function_name)),
    }
}

fn parse_container(args: &[Value]) -> Result<Container, ParseError> {
    expect_length(args, 1)?;
    let raw_fields = args[0].expect_array().with_context("fields")?;

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

    let name = value.lookup_string("name")?.clone();

    Ok(Field { name, ty })
}

fn parse_switch(args: &[Value]) -> Result<Switch, ParseError> {
    expect_length(args, 1)?;

    //   {
    //     "compareTo": "blockId",
    //     "fields": {"-1": "void"},
    //     "default": ["container", []]
    //   }
    let args = args[0].expect_object()?;

    let compare_to = args.lookup_string("compareTo")?.clone();

    let mut variants = IndexMap::new();

    for (key, value) in args.lookup_object("fields")? {
        let ty = parse_type(value).with_context("fields").with_context(key)?;
        let key = key
            .parse::<i64>()
            .map_err(|e| ParseError::new(ErrorKind::ParseInt(e)))?;

        variants.insert(key, ty);
    }

    let default = match args.get("default") {
        Some(d) => {
            let default = parse_type(d).with_context("default")?;
            Some(Box::new(default))
        },
        None => None,
    };

    Ok(Switch {
        compare_to,
        variants,
        default,
    })
}

fn parse_length_prefixed_string(args: &[Value]) -> Result<Type, ParseError> {
    expect_length(args, 1)?;

    let count_type = args[0]
        .expect_object()?
        .lookup("countType")
        .with_context("countType")?;

    let ty = parse_type(count_type).with_context("countType")?;

    Ok(Type::LengthPrefixedString {
        count_type: Box::new(ty),
    })
}

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
              "0x00": "keep_alive",
              "0x01": "chat",
              "0x02": "use_entity",
              "0x03": "flying",
              "0x04": "position",
              "0x05": "look",
              "0x06": "position_look",
              "0x07": "block_dig",
              "0x08": "block_place",
              "0x09": "held_item_slot",
              "0x0a": "arm_animation",
              "0x0b": "entity_action",
              "0x0c": "steer_vehicle",
              "0x0d": "close_window",
              "0x0e": "window_click",
              "0x0f": "transaction",
              "0x10": "set_creative_slot",
              "0x11": "enchant_item",
              "0x12": "update_sign",
              "0x13": "abilities",
              "0x14": "tab_complete",
              "0x15": "settings",
              "0x16": "client_command",
              "0x17": "custom_payload",
              "0x18": "spectate",
              "0x19": "resource_pack_receive"
            }
          }
        ]};

        let _got = parse_type(&doc).unwrap();
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
                Field {
                    name: "name".into(),
                    ty: Type::Named("varint".into()),
                },
                Field {
                    name: "params".into(),
                    ty: Type::Switch(Switch {
                        compare_to: "name".into(),
                        variants: IndexMap::new(),
                        default: None,
                    }),
                },
            ],
        });

        let got = parse_type(&doc).unwrap();

        assert_eq!(got, should_be);
    }
}
