#[macro_use]
extern crate pretty_assertions;

use protodef_codegen::lowering::{Field, Struct, Type};
use serde_json::Value;

#[test]
#[ignore]
fn protocol() {
    let src = include_str!("fixtures/protocol.json");
    let doc: Value = serde_json::from_str(src).unwrap();

    let parsed = protodef_codegen::syntax::parse(&doc).unwrap();
    let analysed = protodef_codegen::lowering::lower(&parsed).unwrap();
    let _rust_code =
        protodef_codegen::backend::generate_rust(&analysed).to_string();
}

#[test]
fn basic() {
    let src = include_str!("fixtures/basic.json");
    let doc: Value = serde_json::from_str(src).unwrap();
    let parsed = protodef_codegen::syntax::parse(&doc).unwrap();
    let analysed = protodef_codegen::lowering::lower(&parsed).unwrap();

    let items_id = analysed.named_types["items"];
    let items_should_be = Type::Struct(Struct {
        fields: vec![
            Field {
                name: "itemCount".into(),
                ty: analysed.named_types["i8"],
            },
            Field {
                name: "itemDamage".into(),
                ty: analysed.named_types["i16"],
            },
        ],
    });
    assert_eq!(analysed.types[&items_id], items_should_be);
    let tokens = protodef_codegen::backend::generate_rust(&analysed);
    let formatted = protodef_codegen::backend::rustfmt(&tokens)
        .unwrap_or_else(|_| tokens.to_string());
    println!("{}", formatted);
}
