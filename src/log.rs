use proc_macro2::TokenStream;

pub(super) fn expand(_: TokenStream) -> TokenStream {
    quote! {
        panic!("lol")
    }
}
