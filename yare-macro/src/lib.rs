//! # Yare
//!
//! Documentation can be found [here](https://github.com/foresterre/yare).
#![deny(clippy::all)]

extern crate proc_macro;
#[macro_use]
extern crate syn;

mod test_cases;

#[proc_macro_attribute]
pub fn parameterized(
    args: ::proc_macro::TokenStream,
    input: ::proc_macro::TokenStream,
) -> ::proc_macro::TokenStream {
    let test_cases = parse_macro_input!(args as test_cases::TestCases);
    let test_fn = parse_macro_input!(input as greenhouse::TestFn);

    test_cases
        .to_token_stream(&test_fn)
        .unwrap_or_else(::syn::Error::into_compile_error)
        .into()
}
