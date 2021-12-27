//! This library provides a derive macro for [`TryFrom<tokio_postgres::Row>`].
//!
//! # Example
//! ```rust
//! #[derive(pg_mapper::TryFromRow)]
//! struct User {
//!     email: String,
//!     password_digest: String,
//! }
//! ```
//!
//! Expanded output:
//! ```rust
//! struct User {
//!     email: String,
//!     password_digest: String,
//! }
//!
//! impl TryFrom<tokio_postgres::Row> for User {
//!     type Error = tokio_postgres::Error;
//!     fn try_from(row: tokio_postgres::Row) -> Result<Self, Self::Error> {
//!         Ok(Self {
//!             email: row.try_get("email")?,
//!             password_digest: row.try_get("password_digest")?,
//!         })
//!     }
//! }
//! ```
pub use pg_mapper_derive::TryFromRow;

#[cfg(test)]
mod tests {
    #[derive(crate::TryFromRow)]
    #[allow(dead_code)]
    struct NamedFields {
        first: String,
        second: i32,
    }
}
