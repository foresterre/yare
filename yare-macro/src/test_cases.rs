use crate::test_fn::TestFn;
use quote::format_ident;
use std::fmt::Formatter;
use syn::braced;
use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;

/// An ordered list of attribute arguments, which consists of test cases which start with the name
/// of the test case, followed by a list of arguments. The order of the argument is equal to the
/// input of the function.
#[derive(Clone)]
pub struct TestCases {
    cases: Punctuated<TestCase, Token![,]>,
}

impl TestCases {
    pub fn to_token_stream(&self, test_fn: &TestFn) -> proc_macro2::TokenStream {
        let visibility = test_fn.visibility();
        let mod_ident = format_ident!("{}", test_fn.identifier());

        let generated_cases = self
            .cases
            .iter()
            .map(|case| case.to_token_stream(test_fn))
            .collect::<Vec<_>>();

        ::quote::quote! {
            #[cfg(test)]
            #visibility mod #mod_ident {
                use super::*;

                #(#generated_cases)*
            }
        }
    }
}

impl std::fmt::Debug for TestCases {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("TestCases(")?;

        for case in self.cases.iter() {
            case.fmt(f)?;
        }

        f.write_str(")")
    }
}

impl Parse for TestCases {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(TestCases {
            cases: Punctuated::parse_terminated(input)?,
        })
    }
}

/// The macro representation of a test case.
/// The syntax for a single test case looks like this `id = { arg1, arg2, ..., argn }`.
/// Here the id is the name of a test case. The list of arguments, which is comma delimited and
/// surrounded by brackets contains a list of arguments which will be supplied to the test function
/// in the same order as provided here.
#[derive(Clone)]
pub struct TestCase {
    id: syn::Ident,
    _assignment: Token![=],
    _braces: syn::token::Brace,
    arguments: Punctuated<syn::Expr, Token![,]>,
}

impl TestCase {
    pub fn to_token_stream(&self, test_fn: &TestFn) -> proc_macro2::TokenStream {
        let attributes = test_fn.attributes();
        let visibility = test_fn.visibility();
        let test_ident = &self.id;

        let bindings = test_fn
            .parameters()
            .zip(&self.arguments)
            .map(|((ident, typ), expr)| {
                ::quote::quote! {
                    let #ident: #typ = #expr;
                }
            });
        let body = test_fn.body();

        ::quote::quote! {
            #[test]
            #(#attributes)*
            #visibility fn #test_ident() {
                #(#bindings)*
                #body
            }
        }
    }
}

impl std::fmt::Debug for TestCase {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("TestCase(id = {:?})", self.id))
    }
}

impl Parse for TestCase {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;

        Ok(TestCase {
            id: input.parse()?,
            _assignment: input.parse()?,
            _braces: braced!(content in input),
            arguments: Punctuated::parse_terminated(&content)?,
        })
    }
}
