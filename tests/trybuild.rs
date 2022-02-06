#[test]
fn trybuild() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/enum.rs");
    t.compile_fail("tests/ui/union.rs");
    t.compile_fail("tests/ui/struct_unit.rs");
    t.compile_fail("tests/ui/struct_empty.rs");
    t.compile_fail("tests/ui/struct_empty_tuple.rs");
    t.pass("tests/ui/struct.rs");
    t.pass("tests/ui/struct_tuple.rs");
}
