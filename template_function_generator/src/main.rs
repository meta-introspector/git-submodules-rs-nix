use std::env;
use wikimedia_template_introspector_core::{parse_template_invocation, TemplateInvocation};
use quote::quote;
use syn::Ident;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <template_string>", args[0]);
        return;
    }

    let template_string = &args[1];

    let invocation = parse_template_invocation(template_string);

    let generated_code = match invocation {
        Some(TemplateInvocation { name, params }) => {
            // Sanitize the template name to be a valid Rust identifier
            let fn_name_str = name.replace(":", "_").replace("-", "_").to_lowercase();
            let fn_name = Ident::new(&fn_name_str, proc_macro2::Span::call_site());

            // Generate function parameters based on template parameters
            let mut fn_params = Vec::new();
            let mut param_assignments = Vec::new();
            for (i, param) in params.iter().enumerate() {
                if param.contains('=') {
                    let parts: Vec<&str> = param.splitn(2, '=').collect();
                    let param_name_str = parts[0].trim().to_lowercase().replace(" ", "_");
                    let param_ident = Ident::new(&param_name_str, proc_macro2::Span::call_site());
                    fn_params.push(quote! { #param_ident: &str });
                    param_assignments.push(quote! { let #param_ident = #param_ident; });
                } else {
                    let param_name_str = format!("param{}", i + 1);
                    let param_ident = Ident::new(&param_name_str, proc_macro2::Span::call_site());
                    fn_params.push(quote! { #param_ident: &str });
                    param_assignments.push(quote! { let #param_ident = #param_ident; });
                }
            }

            quote! {
                pub fn #fn_name(#(#fn_params),*) {
                    #(#param_assignments)*
                    // Placeholder for custom code related to this template
                    println!("Executing template: {}", #name);
                    // You can access parameters like: println!("Param 1: {}", param1);
                }
            }.to_string()
        }
        None => {
            eprintln!("Error: Invalid MediaWiki template syntax for: {}", template_string);
            String::new()
        }
    };

    println!("{}", generated_code);
}
