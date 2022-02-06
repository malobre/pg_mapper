#[derive(pg_mapper::TryFromRow)]
struct MyStruct1 {
    field: bool
}

#[derive(pg_mapper::TryFromRow)]
struct MyStruct2 {
    field0: bool,
    field1: bool
}

fn main() {}
