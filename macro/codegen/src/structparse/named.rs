use super::*;
use conf::StructEntry;
use proc_macro2::TokenStream;


pub fn gen_named_struct(s:&StructEntry, derive_name:&proc_macro2::TokenStream)->TokenStream{
    let struct_name = par_util::to_struct_name(s);
    let  fields_quote = sub_util::gen_kv_tokens(s);
    let  names = sub_util::gen_keys(s);
    //println!("struct_name {:?}",struct_name);
    //println!("derive_name {:?}",derive_name);
    //println!("names {:?}",fields_quote);
    quote! {
        #derive_name
        struct #struct_name{
            #(#fields_quote,)*
        }
        impl #struct_name {
            fn new(#(#fields_quote,)*) -> Self {
                Self{#(#names,)*}
            }
        }
    }

}
