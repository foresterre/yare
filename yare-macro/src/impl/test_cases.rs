use proc_macro2::Ident;
use syn::braced;
use syn::export::Formatter;
use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;

/// An ordered list of attribute arguments, which consists of test cases which start with the name
/// of the test case, followed by a list of arguments. The order of the argument is equal to the
/// input of the function.
#[derive(Clone)]
pub(crate) struct TestCases {
    pub(crate) cases: Punctuated<TestCase, Token![,]>,
}

impl TestCases {
    pub(crate) fn cases(&self) -> Vec<&TestCase> {
        self.cases.iter().collect()
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
pub(crate) struct TestCase {
    pub(crate) id: syn::Ident,
    assignment: Token![=],
    braces: syn::token::Brace,
    pub(crate) arguments: Punctuated<syn::Expr, Token![,]>,
}

impl TestCase {
    pub(crate) fn identifier(&self) -> &Ident {
        &self.id
    }

    pub(crate) fn inputs(&self) -> Vec<&syn::Expr> {
        self.arguments.iter().collect()
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
            assignment: input.parse()?,
            braces: braced!(content in input),
            arguments: Punctuated::parse_terminated(&content)?,
        })
    }
}
