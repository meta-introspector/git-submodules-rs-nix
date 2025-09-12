mod parser_codegen;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};
use wikimedia_template_introspector_core::{parse_template_invocation, TemplateInvocation};

#[proc_macro]
pub fn wikimedia_template(input: TokenStream) -> TokenStream {
    // Parse the input as a string literal
    let template_str = parse_macro_input!(input as LitStr);
    let template_value = template_str.value();

    let invocation = parse_template_invocation(&template_value);

    let expanded = match invocation {
        Some(TemplateInvocation { name, params }) => {
            let name_ident = syn::Ident::new(&name, template_str.span());
            let params_tokens = params.iter().map(|p| quote! { #p.to_string() }).collect::<Vec<_>>();
            quote! {
                pub struct #name_ident {
                    pub params: Vec<String>,
                }

                impl #name_ident {
                    pub fn new() -> Self {
                        #name_ident {
                            params: vec![#(#params_tokens),*],
                        }
                    }
                }
            }
        }
        None => {
            quote! {
                compile_error!("Invalid MediaWiki template syntax");
            }
        }
    };

    expanded.into()
}
