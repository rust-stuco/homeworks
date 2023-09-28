use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::{self, parse_macro_input, spanned::Spanned, DeriveInput, TypeParam};

/// proc macro entrypoint
#[proc_macro_derive(Map)]
pub fn derive_map(annotated_item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_input = parse_macro_input!(annotated_item);
    derive_map_impl(derive_input).into()
}

/// Main implenetation, using `syn` types instead of `proc_macro` types
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

    let all_fields: Vec<_> = match struct_data.fields {
        syn::Fields::Named(ref nf) => nf
            .named
            .iter()
            .map(|f| {
                let field_name = f.ident.as_ref().unwrap();
                quote! { #field_name }
            })
            .collect(),
        _ => {
            return quote_spanned! {
                ident.span() => compile_error!("#[derive(Map)] can only be applied to structs with named fields!");
            }
        }
    };

    // Create function implementation bodies
    let map_inplace_body = quote! {
        #(self.#all_fields.map_inplace(f));*
    };
    let map_body: TokenStream = todo!();

    // Extract the one type parameter from `generics`, returning an error if
    // there are multiple type parameters
    let type_param: syn::TypeParam = todo!();

    // Create a fresh generic type parameter to apply
    let fresh_generic: syn::TypeParam = todo!();
    let generics_with_fresh: syn::Generics = todo!();

    let output = quote! {
        impl #generics_with_fresh Map for #ident #generics
        where #type_param: Map<Item=#fresh_generic>
        {
            type Item = #fresh_generic;
            fn map_inplace(&mut self, f: &mut impl FnMut(&mut Self::Item)) {
                #map_inplace_body
            }
            fn map(self, f: &mut impl FnMut(Self::Item) -> Self::Item) -> Self {
                #map_body
            }
        }
    };

    output.into()
}
