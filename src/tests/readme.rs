use yare::parameterized;

fn add5<T: Into<u32>>(component: T) -> u32 {
    component.into() + 5
}

#[parameterized(
    zero = { 0, 5},
    one = { 1, 6 },
    two = { 2, 7 }
)]
fn test_add5(eh: u16, expected: u32) {
    assert_eq!(add5(eh), expected)
}
