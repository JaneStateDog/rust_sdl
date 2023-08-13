use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse,
    DeriveInput, 
};

#[proc_macro_derive(Component)]
pub fn derive_component(item: TokenStream) -> TokenStream {
    let ast = parse(item).unwrap();

    impl_component(&ast)
}

fn impl_component(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    quote! {
        impl Component for #name {
            fn get_name(&self) -> ComponentName {
                ComponentName(stringify!(#name))
            }
        }
    }.into()
}