
use super::*;
use crate::conf::{Conf, StructEntry};

pub(crate) fn to_ident(name: &str) -> Ident {
    Ident::new(name, proc_macro2::Span::call_site())
}

pub fn to_derive_attr(conf:&Conf)-> proc_macro2::TokenStream{
    let derive_name = to_ident(conf.basic.derive.as_ref());
    quote! {
        #[derive(#derive_name)]
    }
}

pub fn to_struct_name(s:&StructEntry)-> proc_macro2::Ident{
    to_ident(s.name.to_string().to_camel_case().as_ref())
}