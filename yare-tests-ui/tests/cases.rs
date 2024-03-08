#[test]
fn ui() {
    let t = trybuild::TestCases::new();

    t.pass("tests/ok/*.rs");
    t.compile_fail("tests/fail/*.rs");
}
