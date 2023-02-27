use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(ToUrl)]
pub fn to_url_derive(input: TokenStream) -> TokenStream {
    let mut insert_tokens = vec![];
    let parsed_input: DeriveInput = parse_macro_input!(input);
    let struct_name = parsed_input.ident;
    match parsed_input.data {
        Data::Struct(s) => {
            if let Fields::Named(name_fields) = s.fields {
                let a = name_fields.named;
                for field in a {
                    let field = field.ident.as_ref().unwrap();
                    let insert_token = quote! {
                        let mut key_val = String::new();
                        let val = Value::from(self.#field.to_owned());
                        if val.is_object() {
                            let val = serde_json::to_string(&val).unwrap();
                            if val != "null" && val != "" {
                                let key = stringify!(#field);
                                let val = url::form_urlencoded::byte_serialize(val.as_bytes()).collect::<String>();
                                key_val = format!("{}={}", key, val);
                                array.push(key_val);
                            }
                        } else if val.is_number() {
                            let key = stringify!(#field);
                            key_val = format!("{}={}", key, val);
                            array.push(key_val);
                        } else {
                            let val = serde_json::to_string(&val).unwrap();
                            if val != "null"  && val != "" {
                                let key = stringify!(#field);
                                key_val = format!("{}={}", key, val.get(1..val.len() - 1).unwrap());
                                array.push(key_val);
                            }
                        }
                    };
                    insert_tokens.push(insert_token);
                }
            }
        }
        _ => {
            panic!("ToUrl is not yet implemented")
        }
    }
    let tokens = quote! {
        impl ToUrl for #struct_name {
            fn to_url(&self) -> String {
                let mut array = vec![];
                #(#insert_tokens)*
                array.join("&")
            }
        }
    };
    proc_macro::TokenStream::from(tokens)
}