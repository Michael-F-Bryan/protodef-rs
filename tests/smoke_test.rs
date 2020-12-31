use serde_json::Value;

#[test]
#[ignore]
fn parse_the_protocol_file() {
    let src = include_str!("fixtures/protocol.json");
    let doc: Value = serde_json::from_str(src).unwrap();

    let _got = protodef::syntax::parse(&doc).unwrap();
}
