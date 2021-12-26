use proc_macro::{self, TokenStream};
use quote::{quote, quote_spanned};
use syn::{parse_macro_input, spanned::Spanned, Data, DataStruct, DeriveInput, Fields};

#[proc_macro_derive(TryFromRow)]
pub fn derive(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);

    let fields = match data {
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
                    #(#recurse)*
                }
            }
            Fields::Unit | Fields::Unnamed(_) => {
                unimplemented!("Unit & Unnamed struct fields are not supported")
            }
        },
        Data::Enum(_) | Data::Union(_) => unimplemented!("Enum & Union are not supported"),
    };

    let expanded = quote! {
        impl ::core::convert::TryFrom<::tokio_postgres::Row> for #ident {
           type Error = ::tokio_postgres::Error;

            fn try_from(row: ::tokio_postgres::Row) -> Result<Self, Self::Error> {
                Ok(Self {
                    #fields
                })
            }
        }
    };

    TokenStream::from(expanded)
}
