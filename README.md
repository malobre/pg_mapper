# `#[derive(pg_mapper::TryFromRow)]`

# Example
```rust
/// This will try to get each column value by name.
#[derive(pg_mapper::TryFromRow)]
struct User {
    email: String,
    password_digest: String,
}

/// This will try to get each column value by index.
#[derive(pg_mapper::TryFromRow)]
struct Point(i32, i32, i32);
```

Generates:
```rust
impl TryFrom<tokio_postgres::Row> for User {
    type Error = tokio_postgres::Error;
    fn try_from(row: tokio_postgres::Row) -> Result<Self, Self::Error> {
        Ok(Self {
            email: row.try_get("email")?,
            password_digest: row.try_get("password_digest")?,
        })
    }
}

impl TryFrom<tokio_postgres::Row> for Point {
    type Error = tokio_postgres::Error;
    fn try_from(row: tokio_postgres::Row) -> Result<Self, Self::Error> {
        Ok(Self(row.try_get(0)?, row.try_get(1)?, row.try_get(2)?))
    }
}
```
