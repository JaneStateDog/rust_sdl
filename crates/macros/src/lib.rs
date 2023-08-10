extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, 
    ItemFn
};

#[proc_macro]
pub fn get_parameter_types(item: TokenStream) -> TokenStream {
    // Parse the input tokens into a function item
    let input = parse_macro_input!(item as ItemFn);

    // Extract function parameters
    let mut param_info = Vec::new();
    for param in &input.sig.inputs {
        if let syn::FnArg::Typed(pat) = param {
            let ident = &pat.pat;
            let ty = &pat.ty;
            param_info.push(quote! { println!("Name: {} Type: {}", stringify!(#ident), stringify!(#ty)); });
        }
    }

    // Generate code to print parameter names and types
    let expanded = quote! {
        #input

        fn print_params(func: &dyn Fn()) {
            println!("Function Parameters:");
            #(#param_info)*
        }
    };

    TokenStream::from(expanded)
}