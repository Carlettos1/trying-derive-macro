use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::*;

#[proc_macro_derive(Randomizable)]
pub fn derive_randomizable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let generics = add_trait_bounds(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let num = num_enum(&input.data);

    let variants = match input.data {
        Data::Enum(ref data) => data.variants.iter().enumerate().map(|(i, var)| {
            let index = Index::from(i);
            let iden = &var.ident;
            quote_spanned! {var.span()=>
                #index => #name::#iden
            }
        }),
        Data::Struct(_) | Data::Union(_) => unimplemented!(),
    };

    let expanded = quote! {
        impl #impl_generics randomizable::Randomizable for #name #ty_generics #where_clause {
            fn random<R: Rng + ?Sized>(rng: &mut R) -> Self {
                let num = rng.gen_range(0..#num);
                match num {
                    #( #variants, )*
                    _ => panic!("Number not in range of enum"),
                }
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}

fn add_trait_bounds(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param
                .bounds
                .push(parse_quote!(randomizable::Randomizable));
        }
    }
    generics
}

fn num_enum(data: &Data) -> TokenStream {
    match *data {
        Data::Enum(ref data) => {
            let len = data.variants.len();
            quote! {#len}
        }
        Data::Struct(_) | Data::Union(_) => unimplemented!(),
    }
}
