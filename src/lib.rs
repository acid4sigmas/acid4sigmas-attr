extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Lit, Meta, MetaNameValue};

#[proc_macro_derive(TableName, attributes(table_name))]
pub fn table_name_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let name = &ast.ident;

    let mut table_name = String::new();
    for attr in ast.attrs.iter() {
        if let Ok(Meta::NameValue(meta)) = attr.parse_meta() {
            if meta.path.is_ident("table_name") {
                if let Lit::Str(lit_str) = meta.lit {
                    table_name = lit_str.value();
                }
            }
        }
    }

    if table_name.is_empty() {
        panic!("Missing #[table_name] attribute");
    }

    let gen = quote! {
        impl #name {
            pub fn table_name_() -> &'static str {
                #table_name
            }
        }
    };

    gen.into()
}
