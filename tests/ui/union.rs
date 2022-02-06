#[derive(pg_mapper::TryFromRow)]
union MyUnion {
    field: usize
}

fn main() {}
