//! Code generation.

use crate::lowering::{CompilationUnit, TypeId};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use std::{
    collections::HashMap,
    io::{self, Read, Write},
    process::{Child, Command, Stdio},
};

pub fn generate_rust(compilation_unit: &CompilationUnit) -> TokenStream {
    let names = generate_names(compilation_unit);

    compilation_unit
        .types
        .iter()
        .map(|(id, ty)| generate_type_definition(*id, ty, &names))
        .collect()
}

fn generate_names(
    compilation_unit: &CompilationUnit,
) -> HashMap<TypeId, Ident> {
    let mut names = HashMap::new();

    for id in compilation_unit.types.keys() {
        if let Some(real_name) = compilation_unit
            .named_types
            .iter()
            .find(|(_, v)| *v == id)
            .map(|(name, _)| name)
        {
            let ident = Ident::new(real_name, Span::call_site());
            names.insert(*id, ident);
        } else {
            // generate a synthetic name
            let name = id.unique_name();
            let ident = Ident::new(&name, Span::call_site());
            names.insert(*id, ident);
        }
    }

    names
}

fn generate_type_definition(
    id: TypeId,
    ty: &crate::lowering::Type,
    names: &HashMap<TypeId, Ident>,
) -> TokenStream {
    match ty {
        crate::lowering::Type::Native => TokenStream::new(),
        crate::lowering::Type::Struct(s) => {
            generate_struct_definition(id, s, names)
        },
        crate::lowering::Type::Enum(_) => todo!(),
        crate::lowering::Type::LengthPrefixedString(_) => {
            let name = &names[&id];
            quote! {
                #[derive(Debug, Clone, PartialEq)]
                pub struct #name(pub String);
            }
        },
        crate::lowering::Type::BitFields(_) => todo!(),
        crate::lowering::Type::Collection(_) => todo!(),
    }
}

fn generate_struct_definition(
    id: TypeId,
    s: &crate::lowering::Struct,
    names: &HashMap<TypeId, Ident>,
) -> TokenStream {
    let name = &names[&id];
    let fields = s.fields.iter().map(|f| {
        let name = Ident::new(&f.name, Span::call_site());
        let type_name = &names[&f.ty];

        quote! { #name: #type_name, }
    });

    quote! {
        #[derive(Debug, Clone, PartialEq)]
        pub struct #name {
            #( #fields )*
        }
    }
}

pub fn rustfmt(tokens: &TokenStream) -> io::Result<String> {
    let Child { stdin, stdout, .. } = Command::new("rustfmt")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    let mut stdin = stdin.unwrap();
    write!(stdin, "{}", tokens)?;
    stdin.flush()?;
    drop(stdin);

    let mut stdout = stdout.unwrap();
    let mut formatted = String::new();
    stdout.read_to_string(&mut formatted)?;

    Ok(formatted)
}
