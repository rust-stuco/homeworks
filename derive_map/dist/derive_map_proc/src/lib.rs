use proc_macro2::{self, TokenStream};
use quote::{quote, quote_spanned, TokenStreamExt};
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

    let assign_all_fields = match struct_data.fields {
        syn::Fields::Unit => quote! {},
        syn::Fields::Unnamed(uf) => {
            let assignments = (0..uf.unnamed.len()).map(|i| {
                let i = syn::Index::from(i);
                quote! {
                    self.#i.map_inplace(f);
                }
            });
            let mut all_assignments = TokenStream::new();
            all_assignments.append_all(assignments);
            all_assignments
        }
        syn::Fields::Named(nf) => {
            let assignments = nf.named.iter().map(|f| {
                let field_name = f.ident.as_ref().unwrap();
                quote! {
                    self.#field_name.map_inplace(f);
                }
            });
            let mut all_assignments = TokenStream::new();
            all_assignments.append_all(assignments);
            all_assignments
        }
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
        impl #generics Map for #ident #generics {
            type Item = #type_param;
            fn map_inplace(&mut self, f: &mut impl FnMut(&mut Self::Item)) {
                #assign_all_fields
            }
        }
    };

    output.into()
}
