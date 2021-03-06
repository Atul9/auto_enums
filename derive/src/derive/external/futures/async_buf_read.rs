use crate::utils::*;

pub(crate) const NAME: &[&str] = &["futures::AsyncBufRead"];

pub(crate) fn derive(data: &Data, stack: &mut Stack<ItemImpl>) -> Result<()> {
    let io = quote!(::futures::io);

    derive_trait!(
        data,
        parse_quote!(#io::AsyncBufRead)?,
        parse_quote! {
            trait AsyncBufRead {
                #[inline]
                fn poll_fill_buf<'__a>(
                    self: ::core::pin::Pin<&'__a mut Self>,
                    cx: &mut ::core::task::Context<'_>,
                ) -> ::core::task::Poll<::core::result::Result<&'__a [u8], #io::Error>>;
                #[inline]
                fn consume(self: ::core::pin::Pin<&mut Self>, amt: usize);
            }
        }?,
    )
    .map(|item| stack.push(item))
}
