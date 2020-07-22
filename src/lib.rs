extern crate proc_macro;

#[macro_use]
extern crate quote;

use proc_macro2::TokenStream;

mod log;
mod capture;

/**
Logging statements.
*/
#[proc_macro]
pub fn log(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro::TokenStream::from(log::expand(TokenStream::from(item)))
}

/**
Capture a key-value pair using its `Debug` implementation.
*/
#[proc_macro_attribute]
pub fn debug(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    proc_macro::TokenStream::from(capture::expand(
        TokenStream::from(attr),
        TokenStream::from(item),
        quote!(__private_log_capture_from_debug),
    ))
}

/**
Capture a key-value pair using its `Display` implementation.
*/
#[proc_macro_attribute]
pub fn display(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    proc_macro::TokenStream::from(capture::expand(
        TokenStream::from(attr),
        TokenStream::from(item),
        quote!(__private_log_capture_from_display),
    ))
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
