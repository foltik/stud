#![allow(unused)]

use proc_macro::TokenStream;

/////////////////// Args ///////////////////

mod utils;

// #[cfg(feature = "bin")]
// mod derive_args;
// #[cfg(feature = "bin")]
// #[cfg_attr(feature = "bin", proc_macro_derive(Parser))]
// pub fn derive_parser(item: TokenStream) -> TokenStream {
//     utils::expand!(derive_args::derive_args(item.into()))
// }

#[cfg(feature = "bin")]
mod attr_main;
#[cfg(feature = "bin")]
#[cfg_attr(feature = "bin", proc_macro_attribute)]
pub fn main(args: TokenStream, item: TokenStream) -> TokenStream {
    utils::expand!(attr_main::attr_main(args.into(), item.into()))
}
