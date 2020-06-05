extern crate proc_macro;

#[macro_use]
extern crate quote;

use proc_macro2::TokenStream;

#[proc_macro]
pub fn log(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = TokenStream::from(item);

    let output = expand_log(input);

    proc_macro::TokenStream::from(output)
}

fn expand_log(_: TokenStream) -> TokenStream {
    quote! {
        panic!("lol")
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn ui() {
        let t = trybuild::TestCases::new();
        t.pass("tests/ui/pass/*.rs");
        t.compile_fail("tests/ui/fail/*.rs");
    }
}
