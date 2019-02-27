use proc_macro2::TokenStream;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["fmt::Write"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    let fmt = quote!(::core::fmt);

    derive_trait!(
        data,
        parse_quote!(#fmt::Write)?,
        parse_quote! {
            trait Write {
                #[inline]
                fn write_str(&mut self, s: &str) -> #fmt::Result;
                #[inline]
                fn write_char(&mut self, c: char) -> #fmt::Result;
                #[inline]
                fn write_fmt(&mut self, args: #fmt::Arguments<'_>) -> #fmt::Result;
            }
        }?,
    )
}
