

use proc_macro::TokenStream;
use syn::{Data, DeriveInput, Fields, FieldsNamed, parse_macro_input};
use quote::quote;
extern crate proc_macro;



#[proc_macro_derive(Jsonable)]
pub fn derive_jsonable(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let name = &ast.ident;

    let fields = if let Data::Struct(data) = &ast.data {
        &data.fields
    } else {
        return syn::Error::new_spanned(ast.ident, "This macro only supports structs.")
        .to_compile_error()
        .into();
    };

    let named_fields: Vec<_> = if let Fields::Named(FieldsNamed {named,..}) = fields {
        named.iter().collect()
    } else {
        return syn::Error::new_spanned(ast.ident, "This macro only supports structs with named fields")
        .to_compile_error()
        .into()
    };

    let field_names = named_fields.iter().map(|field| &field.ident);
    let field_types = named_fields.iter().map(|field| &field.ty);


    let generated = quote! {
        use http::jsonable::{Parser,JsonValue,FromJsonValue};
        impl Jsonable for #name {

            fn into_json(&self) -> String {
                return String::from("Unimplemented.");
            }

            fn from_json(json_string:&str) -> #name {
                let parsed = Parser::parse_json(json_string).expect("Failed to parse JSON string.");

                let members = if let JsonValue::Object(members) = parsed {
                    members
                } else {
                    panic!("Expected a JSON object for struct {}",stringify!(#name));
                };
                let get_field_val = |key: &str| {
                    members.iter().find(|(k, _)| k == key)
                        .map(|(_, v)| v)
                        .unwrap_or_else(|| panic!("Missing required field '{}'", key))
                };
                return #name {
                    #(
                        #field_names: #field_types::from_json_value(get_field_val(stringify!(#field_names)))
                        .expect(&format!("Failed to convert value for field '{}'",stringify!(#field_names))),
                    )*
                }
            }
        }

        
    };

    generated.into()


}

