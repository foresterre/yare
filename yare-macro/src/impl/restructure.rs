pub(crate) fn impl_case_by_case(
    tests: crate::r#impl::test_cases::TestCases,
    fun: crate::r#impl::fun::TestFn,
) -> ::proc_macro::TokenStream {
    let name = fun.name();
    let vis = fun.vis();
    let mod_name = format!("{}", name);
    let mod_ident = ::syn::Ident::new(mod_name.as_str(), name.span());

    let generated_cases = tests
        .cases()
        .iter()
        .map(|case| {
            generate_test_case(
                case.identifier(),
                &fun.parameters(),
                &case.inputs(),
                fun.body(),
                vis,
                fun.attrs().as_slice(),
            )
        })
        .collect::<Vec<_>>();

    let token_stream = ::quote::quote! {
        #[cfg(test)]
        #vis mod #mod_ident {
            use super::*;

            #(#generated_cases)*
        }
    };

    token_stream.into()
}

fn generate_test_case(
    test_name: &::syn::Ident,
    params: &[(&::syn::Ident, &syn::Type)],
    exprs: &[&::syn::Expr],
    body: &::syn::Block,
    vis: &::syn::Visibility,
    attributes: &[::syn::Attribute],
) -> ::proc_macro2::TokenStream {
    let bindings = (0..params.len()).map(|i| create_binding(params[i], exprs[i]));

    ::quote::quote! {
        #[test]
        #(#attributes)*
        #vis fn #test_name() {
            #(#bindings)*
            #body
        }
    }
}

fn create_binding(
    param: (&::syn::Ident, &::syn::Type),
    expr: &::syn::Expr,
) -> ::proc_macro2::TokenStream {
    let (ident, typ) = param;

    ::quote::quote! {
        let #ident: #typ = #expr;
    }
}
