use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = std::env::args().collect();

    if args.len() != 2 {
        return Err(format!("Usage: {} <filename>", args[0]).into());
    }
    let filename = &args[1];

    let src = std::fs::read_to_string(&filename)?;

    let json = serde_json::from_str(&src)?;
    let parsed = protodef_codegen::syntax::parse(&json)?;
    let analysed = protodef_codegen::lowering::lower(&parsed)?;
    let tokens = protodef_codegen::backend::generate_rust(&analysed);

    let formatted = protodef_codegen::backend::rustfmt(&tokens)
        .unwrap_or_else(|_| tokens.to_string());

    println!("{}", formatted);

    Ok(())
}
