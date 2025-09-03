extern crate proc_macro;
use quote::quote;
use quote::ToTokens;
use syn::token::Token;
use syn::Fields::Named;
use syn::Fields::Unnamed;
use syn::Data::Struct;
use syn::DataStruct;
use syn::DeriveInput;
use syn::parse_macro_input;
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn derive_freezable(_args: TokenStream, input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident,
        data,
        generics,
        ..
    } = parse_macro_input!(input as DeriveInput);
    let where_clause = &generics.where_clause;

    match data {
        Struct(my_struct) => {
            match my_struct.fields {
                Named(fields) => {
                    let field_stream: Vec<proc_macro2::TokenStream> = fields.named.into_iter().map(|field| {
                        quote!{
                            #[serde(default)]
                            field
                        }
                    }).collect();
                    quote!{
                        #[derive(Serialize, Deserialize)]
                        struct #generics #ident #generics #where_clause {
                            #(#field_stream),*
                        }
                        impl Freezable for #ident {}
                    }
                },
                Unnamed(_fields) => panic!("Fields should be named"),
                _unit => panic!("Should not be unit struct"),
            }
        }
        _ => panic!("Must be a struct.")
    }.into()
}

// #[derive(Serialize, Deserialize)]
// struct Example2 {
//     #[serde(default)]
//     pub field1: String,
//     #[serde(default)]
//     pub field2: i8,
//     #[serde(default)]
//     pub field3: bool,
//     #[serde(default)]
//     pub field4: f32,
// }
//
// impl Freezable for Example2 {}
