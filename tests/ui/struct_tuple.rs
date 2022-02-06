#[derive(pg_mapper::TryFromRow)]
struct MyTupleStruct1(bool);

#[derive(pg_mapper::TryFromRow)]
struct MyTupleStruct2(bool, bool);

fn main() {}
