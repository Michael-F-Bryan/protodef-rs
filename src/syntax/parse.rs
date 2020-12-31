use super::{
    errors::{OptionExt, ResultExt, ValueExt},
    Container, Field, ParseError, Protocol, Type,
};
use indexmap::IndexMap;
use serde_json::{Map, Value};

pub fn parse(document: Value) -> Result<Protocol, ParseError> {
    parse_document(document)
}

fn parse_document(document: Value) -> Result<Protocol, ParseError> {
    let mut protocol = Protocol::default();

    if let Some(types) = document.get("types") {
        let types = types.expect_object().with_context("types")?;
        parse_types(types, &mut protocol.types).with_context("types")?;
    }

    Ok(protocol)
}

fn parse_types(
    types: &Map<String, Value>,
    parsed_types: &mut IndexMap<String, Type>,
) -> Result<(), ParseError> {
    for (name, ty) in types {
        let parsed = parse_type(ty, parsed_types).with_context(name)?;
        parsed_types.insert(name.clone(), parsed);
    }

    Ok(())
}

fn parse_type(
    ty: &Value,
    parsed_types: &mut IndexMap<String, Type>,
) -> Result<Type, ParseError> {
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
        "container" => parse_container(args, parsed_types)
            .map(Type::Container)
            .with_context("container"),
        "switch" => todo!(),
        "bitflags" => todo!(),
        "pstring" => todo!(),
        "collection" => todo!(),
        "mapping" => todo!(),
        _ => Err(ParseError::unknown_function(function_name)),
    }
}

fn parse_container(
    args: &[Value],
    parsed_types: &mut IndexMap<String, Type>,
) -> Result<Container, ParseError> {
    if args.is_empty() {
        todo!();
    }
    let raw_fields = args[0].expect_array().with_context("fields")?;

    let mut fields = Vec::new();

    for (_i, arg) in raw_fields.iter().enumerate() {
        let field = parse_field(arg, parsed_types).with_context("field")?;
        fields.push(field);
    }

    Ok(Container { fields })
}

fn parse_field(
    value: &Value,
    parsed_types: &mut IndexMap<String, Type>,
) -> Result<Field, ParseError> {
    let value = value.expect_object()?;

    let ty = value.get("type").or_missing_field("type")?;
    let ty = parse_type(ty, parsed_types).with_context("type")?;

    let name = value
        .get("name")
        .or_missing_field("name")?
        .expect_string()
        .with_context("name")?
        .clone();

    Ok(Field { name, ty })
}

#[cfg(test)]
mod tests {

    use super::*;
    use serde_json::json;

    #[test]
    fn parse_native() {
        let doc = json!("native");
        let mut parsed_types = IndexMap::default();

        let should_be = parse_type(&doc, &mut parsed_types).unwrap();

        assert_eq!(should_be, Type::Native);
    }

    #[test]
    fn parse_named() {
        let doc = json!("u32");
        let mut parsed_types = IndexMap::default();
        let should_be = Type::Named("u32".into());

        let got = parse_type(&doc, &mut parsed_types).unwrap();

        assert_eq!(should_be, should_be);
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
        let mut parsed_types = IndexMap::default();
        let should_be = Type::Container(Container {
            fields: vec![
                Field::new("itemCount", Type::Named("i8".into())),
                Field::new("itemDamage", Type::Named("i16".into())),
                Field::new("nbtData", Type::Named("optionalNbt".into())),
            ]
            .into_iter()
            .collect(),
        });

        let got = parse_type(&doc, &mut parsed_types).unwrap();

        assert_eq!(got, should_be);
    }
}
