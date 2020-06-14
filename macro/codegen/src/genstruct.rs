use super::*;
use crate::conf::StructEntry;


pub struct GenStruct{
    pub name:Ident,
    pub brace_token:token::Brace,
    pub fields:Punctuated<Field, Token![,]>
}

impl Parse for GenStruct{
    fn parse(input:ParseStream)->Result<Self>{
        let fields;
        let name:Ident = input.parse()?;
        input.parse::<Token![->]>()?;
        let brace_token = braced!(fields in input);
        let fields = fields.parse_terminated(Field::parse_named)?;
        Ok(GenStruct {
            name,
            brace_token,
            fields,
        })
    }
}


pub fn struct_to_tokenstream(input:TokenStream)->TokenStream{
    let GenStruct{
        name,
        brace_token:_,
        fields
    } = parse_macro_input!(input as GenStruct);

    let fields_quote =
        fields.iter().map(|field|{
            let name = &field.ident;
            let ty = &field.ty;
            quote!(#name:#ty)
        });

    let args_quote =
        fields.iter().map(|field|{
            let name = &field.ident;
            let ty = &field.ty;
            quote!(#name: #ty)
        });

    let args_name_quote = fields.iter().map(
        |field| {
            let name = &field.ident;
            quote!(#name)
        }
    );

    let struct_name = util::to_ident(name.to_string().to_camel_case().as_ref());

    let expanded = quote! {

            #[derive(Debug)]
            pub struct #struct_name {
                #( #fields_quote, )*
            }

            impl #struct_name {
                pub fn new( #(#args_quote,)* ) -> Self {
                    #struct_name { #(#args_name_quote,)* }
                 }
            }

    };

    TokenStream::from(expanded)
}


pub struct ConfPath{
    pub dir:Ident,
    pub file:Ident
}



fn get_structs_quote(s:&StructEntry, derive_name:&proc_macro2::TokenStream)->proc_macro2::TokenStream{
    match s.tagged.as_str() {
        "named"=>structparse::named::gen_named_struct(&s, derive_name),
        "unit"=>structparse::unit::gen_unit_struct(&s, derive_name),
        "tuple"=>structparse::tuple::gen_tuple_struct(&s, derive_name),
        _=>panic!("error tagged")
    }


}

pub(crate) fn struct_by_conf_to_tokenstream(input: TokenStream) -> TokenStream {

    let conf = conf::Conf::read_config();
    let derive_name = util::to_derive_attr(&conf);
    let structs_quote = conf.structs.iter().map(|s| {
        get_structs_quote(s,&derive_name)
    });


    let expanded = quote! {
            #( #structs_quote )*
    };

    TokenStream::from(expanded)
}
