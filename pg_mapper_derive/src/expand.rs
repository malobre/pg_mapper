use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::{spanned::Spanned, Data, DataStruct, DeriveInput, Error, Fields};

pub fn try_from_row(input: DeriveInput) -> Result<TokenStream, Error> {
    let DeriveInput {
        ref ident,
        ref data,
        ..
    } = input;

    let body = match data {
        Data::Struct(DataStruct { fields, .. }) => match fields {
            Fields::Named(ref fields) => {
                let recurse = fields.named.iter().map(|field| {
                    let name = &field.ident;
                    let name_str = name.clone().map(|ident| ident.to_string());
                    quote_spanned! { field.span() =>
                        #name: row.try_get(#name_str)?,
                    }
                });

                quote! {
                    Ok(Self {
                        #(#recurse)*
                    })
                }
            }
            Fields::Unnamed(ref fields) => {
                let recurse = fields.unnamed.iter().enumerate().map(|(index, field)| {
                    quote_spanned! { field.span() =>
                        row.try_get(#index)?,
                    }
                });

                quote! {
                    Ok(Self(
                        #(#recurse)*
                    ))
                }
            }
            bad @ Fields::Unit => {
                return Err(Error::new_spanned(bad, "unit structs are not supported"))
            }
        },
        Data::Enum(_) => return Err(Error::new_spanned(input, "enums are not supported")),
        Data::Union(_) => return Err(Error::new_spanned(input, "unions are not supported")),
    };

    let expanded = quote! {
        impl ::core::convert::TryFrom<::tokio_postgres::Row> for #ident {
           type Error = ::tokio_postgres::Error;

            fn try_from(row: ::tokio_postgres::Row) -> Result<Self, Self::Error> {
                #body
            }
        }
    };

    Ok(expanded)
}
