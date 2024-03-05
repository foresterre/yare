use syn::spanned::Spanned;

pub struct TestFn {
    fun: ::syn::ItemFn,
}

impl TestFn {
    pub fn attributes(&self) -> &[::syn::Attribute] {
        &self.fun.attrs
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
                        Err(syn::Error::new(pat.span(), "Expected identifier"))
                    }
                } else {
                    Err(syn::Error::new(item.span(), "Expected function argument"))
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
