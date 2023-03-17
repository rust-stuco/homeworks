use proc_macro2::{self, TokenStream};
use quote::{quote, quote_spanned};
use syn::{self, parse_macro_input, spanned::Spanned, DeriveInput, TypeParam};

#[proc_macro_derive(Map)]
pub fn derive_map(annotated_item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_input = parse_macro_input!(annotated_item);
    derive_map_impl(derive_input).into()
}

fn derive_map_impl(annotated_item: DeriveInput) -> TokenStream {
    let DeriveInput {
        ident,
        generics,
        data,
        ..
    } = annotated_item;

    let struct_data = match data {
        syn::Data::Struct(s) => s,
        _ => {
            return quote_spanned! {
                ident.span() => compile_error!("#[derive(Map)] can only be applied to `struct`!");
            }
        }
    };

    let all_fields = match struct_data.fields {
        syn::Fields::Unit => Vec::new(),
        syn::Fields::Unnamed(ref uf) => (0..uf.unnamed.len())
            .map(|i| {
                let i = syn::Index::from(i);
                quote! { #i }
            })
            .collect(),
        syn::Fields::Named(ref nf) => nf
            .named
            .iter()
            .map(|f| {
                let field_name = f.ident.as_ref().unwrap();
                quote! { #field_name }
            })
            .collect(),
    };

    let map_inplace_body = quote! {
        #(self.#all_fields.map_inplace(f));*
    };

    let map_body = match struct_data.fields {
        syn::Fields::Unit => quote! {},
        syn::Fields::Unnamed(_) => quote! {
            Self(#(self.#all_fields.map(f)),*)
        },
        syn::Fields::Named(_) => quote! {
            Self {
                #(#all_fields: self.#all_fields.map(f)),*
            }
        },
    };

    let mut type_param = Option::<TypeParam>::None;
    for param in generics.params.iter() {
        if let syn::GenericParam::Type(tp) = param {
            match type_param {
                None => type_param = Some(tp.clone()),
                Some(_) => {
                    return quote_spanned! {
                        generics.span() => compile_error!("#[derive(Map)] cannot be applied to structs with more than one type parameter");
                    }
                }
            }
        }
    }

    let type_param = match type_param {
        Some(tp) => tp,
        None => {
            return quote_spanned! {
                ident.span() => compile_error!("#[derive(Map)] must be applied to a struct with a single type parameter");
            }
        }
    };

    let output = quote! {
        impl #generics Map for #ident #generics
        where #type_param: Map<Item=#type_param>
        {
            type Item = #type_param;
            fn map_inplace(&mut self, f: &mut impl FnMut(&mut Self::Item)) {
                #map_inplace_body
            }
            fn map(self, mut f: impl FnMut(Self::Item) -> Self::Item) -> Self {
                #map_body
            }
        }
    };

    output.into()
}
