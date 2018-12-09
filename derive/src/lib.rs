#![crate_type = "proc-macro"]
#![recursion_limit = "256"]
#![doc(html_root_url = "https://docs.rs/auto_enumerate_derive/0.1.0")]

extern crate lazy_static;
extern crate proc_macro;
extern crate proc_macro2;
extern crate quote;
extern crate smallvec;
extern crate syn;

#[macro_use]
mod utils;

mod attribute;
mod derive;

use proc_macro::TokenStream;

/// An attribute macro like a wrapper of `#[derive]`, implementing
/// the supported traits and passing unsupported traits to `#[derive]`.
#[proc_macro_attribute]
pub fn enum_derive(args: TokenStream, input: TokenStream) -> TokenStream {
    crate::attribute::attribute(args, input)
}
