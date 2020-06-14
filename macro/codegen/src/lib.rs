extern crate proc_macro;

use{
    heck,
    quote::*,
    self::proc_macro::TokenStream,
    syn::parse::{Parse,ParseStream, Result},
    syn::{Ident, token, punctuated::Punctuated, Field, Token, braced, parse_macro_input}
};
use heck::CamelCase;
use crate::conf::Conf;
mod genstruct;
mod util;
mod conf;
mod structparse;

const CONF_FILE_PATH : &'static str = "confs/method.toml";

#[proc_macro]
pub fn gen_struct(input: TokenStream) -> TokenStream {

    genstruct::struct_to_tokenstream(input)
}

#[proc_macro]
pub fn gen_struct_by_conf(_input: TokenStream) -> TokenStream {
    genstruct::struct_by_conf_to_tokenstream(_input)
}




// #[macro_export]
// macro_rules! gen_derive_name {
//     ($arg:expr) => {
//     util::to_ident($arg.basic.derive.as_ref());};
// }