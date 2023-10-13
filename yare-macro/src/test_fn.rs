pub struct TestFn {
    fun: ::syn::ItemFn,
}

impl TestFn {
    pub fn parameters(&self) -> impl Iterator<Item = (&::syn::Ident, &::syn::Type)> {
        if self.fun.sig.inputs.is_empty() {}

        self.fun.sig.inputs.iter().filter_map(|item| {
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
    }

    pub fn attributes(&self) -> &[::syn::Attribute] {
        &self.fun.attrs
    }

    pub fn visibility(&self) -> &::syn::Visibility {
        &self.fun.vis
    }

    pub fn identifier(&self) -> &::syn::Ident {
        &self.fun.sig.ident
    }

    pub fn body(&self) -> &::syn::Block {
        &self.fun.block
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
