use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(Component)]
pub fn component_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_component(&ast)
}

fn impl_component(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl Component for #name {
            fn get_name(&self) -> String {
                String::from(stringify!(#name))
            }
        }
    };
    gen.into()
}