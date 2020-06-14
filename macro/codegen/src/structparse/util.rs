use proc_macro2::TokenStream;
use super::*;
use conf::StructEntry;



pub fn gen_kv_tokens(s:&StructEntry) ->Vec<TokenStream>{
    let mut args = vec![];
    for m in s.fields.as_ref().unwrap(){
        for (k, v) in m{
            let name = par_util::to_ident(k);
            let ty = par_util::to_ident(v.as_str().unwrap());
            args.push(quote!(#name: #ty))
        }
    }
    args
}


pub fn gen_keys(s:&StructEntry) ->Vec<TokenStream>{
    let mut args = vec![];
    for m in s.fields.as_ref().unwrap(){
        for (k, _v) in m{
            let name = par_util::to_ident(k);
            args.push(quote!(#name))
        }
    }
    args
}

pub fn gen_values(s:&StructEntry) ->Vec<TokenStream>{
    let mut args = vec![];
    for m in s.fields.as_ref().unwrap(){
        for (_k, v) in m {
            use toml::value::Value;
            let ty = par_util::to_ident(v.as_str().unwrap());
            args.push(quote!(#ty))
        }
    }
    args
}