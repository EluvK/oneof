use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Type, parse_macro_input, spanned::Spanned};

#[proc_macro_derive(OneOf)]
pub fn derive_oneof(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let ident = input.ident.clone();
    let generics = input.generics.clone();
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // Only support structs
    let fields = match input.data {
        Data::Struct(s) => s.fields,
        _rest => {
            return syn::Error::new(input.ident.span(), "OneOf can only be derived for structs")
                .to_compile_error()
                .into();
        }
    };

    // Collect validation code for Option fields
    let mut counters = Vec::<proc_macro2::TokenStream>::new();

    // Only support structs with named fields
    let Fields::Named(named) = &fields else {
        return syn::Error::new(
            ident.span(),
            "OneOf: this struct has no named fields to check",
        )
        .to_compile_error()
        .into();
    };
    for f in named.named.iter() {
        if !is_option_type(&f.ty) {
            return syn::Error::new(f.ty.span(), "OneOf: all fields must be of type Option<T>")
                .to_compile_error()
                .into();
        }
        let name = f.ident.as_ref().unwrap();
        counters.push(quote! {
            if self.#name.is_some() { count += 1; }
        });
    }

    if counters.is_empty() {
        return syn::Error::new(
            ident.span(),
            "OneOf: this struct has no Option<_> fields to check",
        )
        .to_compile_error()
        .into();
    }

    let expanded = quote! {
        impl #impl_generics #ident #ty_generics #where_clause {
            /// Returns Ok(()) if and only if exactly one Option<_> field is Some.
            pub fn validate_oneof(&self) -> Result<(), &'static str> {
                let mut count: usize = 0;
                #(#counters)*
                if count == 1 {
                    Ok(())
                } else {
                    Err("Exactly one field must be Some")
                }
            }

            /// Returns the number of Option<_> fields that are Some.
            pub fn oneof_count(&self) -> usize {
                let mut count: usize = 0;
                #(#counters)*
                count
            }
        }
    };

    TokenStream::from(expanded)
}

/// Determines whether the type is Option<_> (supports Option<T> or std::option::Option<T>)
fn is_option_type(ty: &Type) -> bool {
    let Type::Path(tp) = ty else {
        return false;
    };
    let Some(seg) = tp.path.segments.last() else {
        return false;
    };
    if seg.ident != "Option" {
        return false;
    }
    matches!(&seg.arguments, syn::PathArguments::AngleBracketed(_))
}
