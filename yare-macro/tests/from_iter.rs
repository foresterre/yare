#[test]
fn from_iter_failures() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/from_iter/fail/*.rs");
}

#[test]
fn from_iter_pass() {
    let t = trybuild::TestCases::new();
    t.compile_pass("tests/from_iter/pass/*.rs");
}
