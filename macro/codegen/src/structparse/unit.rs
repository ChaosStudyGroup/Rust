use super::*;
use conf::StructEntry;
use proc_macro2::TokenStream;


pub fn gen_unit_struct(s:&StructEntry, derive_name:&proc_macro2::TokenStream)->TokenStream{
    let struct_name = par_util::to_struct_name(s);

    quote! {
        #derive_name
        struct #struct_name;
        impl #struct_name {
            fn new() -> Self {
                Self
            }
        }
    }

}
