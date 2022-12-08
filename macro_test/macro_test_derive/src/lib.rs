use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(MyMacro)]
pub fn my_macro_derive(input: TokenStream) -> TokenStream {
    // will be called when a user of lib specifies #[derive(MyMacro)] on a type

    // parse TokenStream; construct a representation of Rust code as a syntax tree
    let syntree = syn::parse(input).unwrap();

    // transform syntax tree; build the trait implementation
    impl_my_macro(&syntree)
}

fn impl_my_macro(syntree: &syn::DeriveInput) -> TokenStream {
    let name = &syntree.ident;
    let gen = quote! {
        impl MyMacro for #name {
            fn my_macro() {
                println!("This is {} speaking, thanks to proc macro!", stringify!(#name));
            }
        }
    };
    gen.into()
}
