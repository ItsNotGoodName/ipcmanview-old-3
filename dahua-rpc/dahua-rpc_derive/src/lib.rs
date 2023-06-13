use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parser, parse_macro_input, DeriveInput, Fields};

#[proc_macro_derive(ConfigTable)]
pub fn config_table_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_config_table(&ast)
}

fn impl_config_table(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let name_string = name.to_string();

    let set_expr = if should_add_set(ast) {
        quote!(
            pub async fn set(self, rpc: crate::RequestBuilder) -> Result<(), crate::Error> {
                crate::modules::configmanager::SetConfigRequest {name: #name_string, channel: 0, table: self}.set(rpc).await
            }
        )
    } else {
        quote!()
    };

    let gen = quote! {
        impl #name {
            pub async fn get(rpc: crate::RequestBuilder) -> Result<Self, crate::Error> {
                crate::modules::configmanager::GetConfigRequest { name: #name_string, channel: 0}.get::<Self>(rpc).await
            }

            pub async fn get_default(rpc: crate::RequestBuilder) -> Result<Self, crate::Error> {
                crate::modules::configmanager::GetConfigRequest { name: #name_string, channel: 0}.get_default::<Self>(rpc).await
            }

            #set_expr
        }
    };

    TokenStream::from(gen)
}

fn should_add_set(ast: &syn::DeriveInput) -> bool {
    if let syn::Data::Struct(data) = &ast.data {
        match &data.fields {
            Fields::Named(fields) => {
                for ele in fields.named.iter() {
                    // TODO: recursive check structs to make sure they have serde(flatten)
                    for ele in ele.attrs.iter() {
                        if let Some(ident) = ele.path.get_ident() {
                            if ident == "serde" {
                                if ele.tokens.to_string() == "(flatten)" {
                                    return true;
                                }
                            }
                        }
                    }
                }
            }
            Fields::Unnamed(_) | Fields::Unit => {
                return true;
            }
        }
    }

    return false;
}

#[proc_macro_attribute]
pub fn config_table(_: TokenStream, input: TokenStream) -> TokenStream {
    let mut ast: DeriveInput = parse_macro_input!(input as DeriveInput);

    if let syn::Data::Struct(ref mut data) = ast.data {
        match data.fields {
            Fields::Named(ref mut fields) => {
                let field = syn::Field::parse_named
                    .parse2(quote! { #[serde(flatten)]
                    pub extra: std::collections::HashMap<String, serde_json::Value>
                    })
                    .unwrap();
                fields.named.push(field);
            }
            Fields::Unnamed(_) | Fields::Unit => {}
        }
    }

    let gen = quote! {
        #ast
    };

    TokenStream::from(gen)
}
