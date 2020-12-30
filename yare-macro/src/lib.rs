//! # Yare
//!

#[macro_use]
extern crate syn;
extern crate proc_macro;

mod r#impl;

#[proc_macro_attribute]
pub fn parameterized(
    args: ::proc_macro::TokenStream,
    input: ::proc_macro::TokenStream,
) -> ::proc_macro::TokenStream {
    let tests = parse_macro_input!(args as r#impl::test_cases::TestCases);
    let fun = parse_macro_input!(input as r#impl::fun::TestFn);

    r#impl::restructure::impl_case_by_case(tests, fun)
}
