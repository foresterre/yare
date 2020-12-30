pub(crate) struct TestFn {
    fun: ::syn::ItemFn,
}

impl TestFn {
    pub(crate) fn parameters(&self) -> ::std::vec::Vec<(&::syn::Ident, &::syn::Type)> {
        self.fun
            .sig
            .inputs
            .iter()
            .filter_map(|item| {
                if let ::syn::FnArg::Typed(::syn::PatType { pat, ty, .. }) = item {
                    if let ::syn::Pat::Ident(::syn::PatIdent { ident, .. }) = pat.as_ref() {
                        Some((ident, ty.as_ref()))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect()
    }

    pub(crate) fn name(&self) -> &::syn::Ident {
        &self.fun.sig.ident
    }

    pub(crate) fn vis(&self) -> &::syn::Visibility {
        &self.fun.vis
    }

    pub(crate) fn body(&self) -> &::syn::Block {
        &self.fun.block
    }

    pub(crate) fn attrs(&self) -> &Vec<::syn::Attribute> {
        &self.fun.attrs
    }
}

impl ::std::fmt::Debug for TestFn {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("TestFn(")?;

        for param in self.parameters() {
            f.write_str(&format!("{:?}, ", param.0))?;
        }

        f.write_str(")")
    }
}

impl ::syn::parse::Parse for TestFn {
    fn parse(input: ::syn::parse::ParseStream) -> ::syn::parse::Result<Self> {
        Ok(TestFn {
            fun: input.parse()?,
        })
    }
}
