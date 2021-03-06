use crate::utils::*;

pub(crate) const NAME: &[&str] = &["futures::AsyncSeek"];

pub(crate) fn derive(data: &Data, stack: &mut Stack<ItemImpl>) -> Result<()> {
    let io = quote!(::futures::io);

    derive_trait!(
        data,
        parse_quote!(#io::AsyncSeek)?,
        parse_quote! {
            trait AsyncSeek {
                #[inline]
                fn poll_seek(
                    self: ::core::pin::Pin<&mut Self>,
                    cx: &mut ::core::task::Context<'_>,
                    pos: ::std::io::SeekFrom,
                ) -> ::core::task::Poll<::core::result::Result<u64, #io::Error>>;
            }
        }?,
    )
    .map(|item| stack.push(item))
}
