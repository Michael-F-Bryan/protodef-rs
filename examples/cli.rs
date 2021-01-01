use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = std::env::args().collect();

    if args.len() != 2 {
        return Err(format!("Usage: {} <filename>", args[0]).into());
    }
    let filename = &args[1];

    let src = std::fs::read_to_string(&filename)?;

    let json = serde_json::from_str(&src)?;
    let parsed = protodef::syntax::parse(&json)?;
    let analysed = protodef::lowering::lower(&parsed)?;
    let tokens = protodef::backend::generate_rust(&analysed);

    let formatted = protodef::backend::rustfmt(&tokens)
        .unwrap_or_else(|_| tokens.to_string());

    println!("{}", formatted);

    Ok(())
}
