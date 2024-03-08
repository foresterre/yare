use syn::spanned::Spanned;

pub struct TestFn {
    attributes: Vec<Attribute>,
    fun: ::syn::ItemFn,
}

impl TestFn {
    pub fn assert_at_most_one_test_macro(&self) -> ::syn::Result<()> {
        let test_macros = self
            .attributes
            .iter()
            .filter_map(Attribute::to_test_macro)
            .collect::<Vec<_>>();

        let count = test_macros.len();

        if count <= 1 {
            Ok(())
        } else {
            let meta = &test_macros[count - 1];

            Err(::syn::Error::new(
                meta.span(),
                format_args!(
                    "Expected at most 1 #[test_macro(...)] attribute, but {} were given",
                    count
                ),
            ))
        }
    }

    pub fn attributes(&self) -> Vec<::syn::Attribute> {
        let mut parsed_attr = self
            .attributes
            .iter()
            .filter_map(Attribute::to_normal)
            .collect::<Vec<_>>();

        parsed_attr.extend(self.fun.attrs.iter().cloned());

        parsed_attr
    }

    pub fn test_macro_attribute(&self) -> ::syn::Meta {
        self.attributes
            .iter()
            .find_map(Attribute::to_test_macro)
            .unwrap_or_else(|| {
                ::syn::Meta::Path(::syn::Path::from(::syn::Ident::new(
                    "test",
                    ::proc_macro2::Span::call_site(),
                )))
            })
    }

    pub fn visibility(&self) -> &::syn::Visibility {
        &self.fun.vis
    }

    pub fn constness(&self) -> Option<&::syn::token::Const> {
        self.fun.sig.constness.as_ref()
    }

    pub fn asyncness(&self) -> Option<&::syn::token::Async> {
        self.fun.sig.asyncness.as_ref()
    }

    pub fn unsafety(&self) -> Option<&::syn::token::Unsafe> {
        self.fun.sig.unsafety.as_ref()
    }

    pub fn abi(&self) -> Option<&::syn::Abi> {
        self.fun.sig.abi.as_ref()
    }

    pub fn identifier(&self) -> &::syn::Ident {
        &self.fun.sig.ident
    }

    pub fn parameters(&self) -> ::syn::Result<Vec<(&::syn::Ident, &::syn::Type)>> {
        self.fun
            .sig
            .inputs
            .iter()
            .map(|item| {
                if let ::syn::FnArg::Typed(::syn::PatType { pat, ty, .. }) = item {
                    if let ::syn::Pat::Ident(::syn::PatIdent { ident, .. }) = pat.as_ref() {
                        Ok((ident, ty.as_ref()))
                    } else {
                        Err(::syn::Error::new(pat.span(), "Expected identifier"))
                    }
                } else {
                    Err(::syn::Error::new(item.span(), "Expected function argument"))
                }
            })
            .collect::<::syn::Result<_>>()
    }

    pub fn return_type(&self) -> &::syn::ReturnType {
        &self.fun.sig.output
    }

    pub fn body(&self) -> &::syn::Block {
        &self.fun.block
    }
}

impl ::syn::parse::Parse for TestFn {
    fn parse(input: ::syn::parse::ParseStream) -> ::syn::parse::Result<Self> {
        Ok(TestFn {
            attributes: input
                .call(::syn::Attribute::parse_outer)?
                .into_iter()
                .map(|attr| {
                    if attr.path().is_ident("test_macro") {
                        attr.parse_args::<::syn::Meta>().map(Attribute::TestMacro)
                    } else {
                        Ok(Attribute::Normal(attr))
                    }
                })
                .collect::<::syn::Result<Vec<_>>>()?,
            fun: input.parse()?,
        })
    }
}

impl ::std::fmt::Debug for TestFn {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("TestFn(")?;

        let parameters = self.parameters().map_err(|_| ::std::fmt::Error)?;

        for param in parameters {
            f.write_fmt(format_args!("{:?}, ", param.0))?;
        }

        f.write_str(")")
    }
}

enum Attribute {
    /// A regular attribute, which isn't named "test_macro"
    Normal(::syn::Attribute),
    // An attribute named "test_macro"
    TestMacro(::syn::Meta),
}

impl Attribute {
    fn to_normal(&self) -> Option<::syn::Attribute> {
        match self {
            Attribute::Normal(inner) => Some(inner.clone()),
            _ => None,
        }
    }

    fn to_test_macro(&self) -> Option<::syn::Meta> {
        match self {
            Attribute::TestMacro(inner) => Some(inner.clone()),
            _ => None,
        }
    }
}
