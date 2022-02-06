//! This library provides a derive macro for [`TryFrom<tokio_postgres::Row>`].
//!
//! # Example
//! ```rust
//! /// This will try to get each column value by name.
//! #[derive(pg_mapper::TryFromRow)]
//! struct User {
//!     email: String,
//!     password_digest: String,
//! }
//!
//! /// This will try to get each column value by index.
//! #[derive(pg_mapper::TryFromRow)]
//! struct Point(i32, i32, i32);
//! ```
//!
//! Generates:
//! ```rust
//! # struct User {
//! #     email: String,
//! #     password_digest: String,
//! # }
//! impl TryFrom<tokio_postgres::Row> for User {
//!     type Error = tokio_postgres::Error;
//!     fn try_from(row: tokio_postgres::Row) -> Result<Self, Self::Error> {
//!         Ok(Self {
//!             email: row.try_get("email")?,
//!             password_digest: row.try_get("password_digest")?,
//!         })
//!     }
//! }
//!
//! # struct Point(i32, i32, i32);
//! impl TryFrom<tokio_postgres::Row> for Point {
//!     type Error = tokio_postgres::Error;
//!     fn try_from(row: tokio_postgres::Row) -> Result<Self, Self::Error> {
//!         Ok(Self(row.try_get(0)?, row.try_get(1)?, row.try_get(2)?))
//!     }
//! }
//! ```
use proc_macro::TokenStream;
use syn::parse_macro_input;

mod expand;

#[proc_macro_derive(TryFromRow)]
pub fn derive_try_from_row(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);

    expand::try_from_row(input)
        .unwrap_or_else(|error| error.to_compile_error())
        .into()
}
