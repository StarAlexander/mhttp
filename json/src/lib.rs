use proc_macro::TokenStream;
use syn::{Data, DeriveInput, Fields, FieldsNamed, parse_macro_input};
use quote::quote;

#[proc_macro_derive(Jsonable)]
pub fn derive_jsonable(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let name = &ast.ident;

    let fields = if let Data::Struct(data) = &ast.data {
        &data.fields
    } else {
        return syn::Error::new_spanned(
            ast.ident, 
            "Jsonable can only be derived for structs"
        )
        .to_compile_error()
        .into();
    };

    let named_fields: Vec<_> = if let Fields::Named(FieldsNamed { named, .. }) = fields {
        named.iter().collect()
    } else {
        return syn::Error::new_spanned(
            ast.ident, 
            "Jsonable can only be derived for structs with named fields"
        )
        .to_compile_error()
        .into()
    };

    // Generate serialization code for each field
    let serialize_fields = named_fields.iter().map(|field| {
        let field_name = &field.ident;
        let field_type = &field.ty;
        quote! {
            {
                let field_value = &self.#field_name;
                let field_str = match stringify!(#field_type) {
                    "String" => format!("\"{}\"", escape_json_string(&field_value.to_string())),
                    "f64" => field_value.to_string(),
                    "bool" => field_value.to_string(),
                    "Vec" => field_value.to_string(),
                    _ => panic!("Unexpected type.")
                };
                parts.push(format!("\"{}\":{}", stringify!(#field_name), field_str));
            }
        }
    });

    // Generate deserialization code for each field
    let deserialize_fields = named_fields.iter().map(|field| {
        let field_name = &field.ident;
        let field_type = &field.ty;
        
        quote! {
            #field_name: {
                let field_json = get_field_val(stringify!(#field_name))?;
                #field_type::from_json_value(field_json)
                    .map_err(|e| format!("Failed to convert field '{}' from JSON: {}", stringify!(#field_name), e))?
            },
        }
    });

    let generated = quote! {
        use http::jsonable::{Parser, JsonValue, FromJsonValue};

        // Helper function for escaping JSON strings
        fn escape_json_string(s: &str) -> String {
            s.replace('\\', "\\\\")
             .replace('"', "\\\"")
             .replace('\n', "\\n")
             .replace('\r', "\\r")
             .replace('\t', "\\t")
             .replace('\x08', "\\b")
             .replace('\x0c', "\\f")
        }

        impl Jsonable for #name {
            fn into_json(&self) -> String {
                let mut parts = Vec::new();
                
                #(#serialize_fields)*
                
                format!("{{{}}}", parts.join(","))
            }

            fn from_json(json_string: &str) -> Result<Self, Box<dyn std::error::Error>> {
                let parsed = Parser::parse_json(json_string)
                    .map_err(|e| format!("Failed to parse JSON: {}", e))?;

                let members = if let JsonValue::Object(members) = parsed {
                    members
                } else {
                    return Err(format!("Expected a JSON object for struct {}", stringify!(#name)).into());
                };

                let get_field_val = |key: &str| -> Result<&JsonValue, Box<dyn std::error::Error>> {
                    members.iter()
                        .find(|(k, _)| k == key)
                        .map(|(_, v)| v)
                        .ok_or_else(|| format!("Missing required field '{}'", key).into())
                };

                Ok(#name {
                    #(#deserialize_fields)*
                })
            }
        }
    };

    generated.into()
}