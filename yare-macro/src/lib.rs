//! # Yare
//!

#[macro_use]
extern crate syn;
extern crate proc_macro;

mod test_cases;
mod test_fn;

#[proc_macro_attribute]
pub fn parameterized(
    args: ::proc_macro::TokenStream,
    input: ::proc_macro::TokenStream,
) -> ::proc_macro::TokenStream {
    let test_cases = parse_macro_input!(args as test_cases::TestCases);
    let test_fn = parse_macro_input!(input as test_fn::TestFn);

    test_cases
        .to_token_stream(&test_fn)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
