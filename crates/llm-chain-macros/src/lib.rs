extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{
    parse_macro_input, DeriveInput, Ident, LitStr,
    __private::{quote::quote, TokenStream2},
};

fn literal_from_ident(ident: Ident) -> LitStr {
    let value = ident.to_string();
    let span = ident.span();
    LitStr::new(&value, span)
}

#[proc_macro_derive(Describe, attributes(purpose))]
pub fn derive_describe(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let syn::Data::Struct(described_struct) = input.data else {
        panic!("Can only describe structs")
    };

    // Parse field attrs
    let pairs: Vec<(LitStr, LitStr)> = described_struct
        .fields
        .iter()
        .map(|field| {
            let ident = field
                .ident
                .clone()
                .expect("All struct fields must be named");
            (
                literal_from_ident(ident),
                field
                    .attrs
                    .iter()
                    .filter(|attr| {
                        attr.path().segments.len() == 1
                            && attr.path().segments[0].ident == "purpose"
                    })
                    .nth(0)
                    .expect("All fields on the string must have a purpose annotation")
                    .parse_args::<LitStr>()
                    .expect("Purpose must be a single string literal"),
            )
        })
        .collect();
    if pairs.len() == 0 {
        panic!("You need to annotate each field");
    }

    // Generate FormatParts from fields
    let mut format_parts = Vec::new();
    for (key, purpose) in pairs.into_iter() {
        let gen: TokenStream2 = quote! {
            FormatPart {
                key: #key.to_string(),
                purpose: #purpose.to_string()
            }
        }
        .into();

        format_parts.push(gen);
    }

    // Implement trait using generated FormatParts
    let name = &input.ident;

    let gen: TokenStream = quote! (
        impl Describe for #name {
            fn describe() -> Format {
                 Format {
                    parts: vec![
                        #(#format_parts),*
                    ]
                }
            }
        }
    )
    .into();

    gen
}
