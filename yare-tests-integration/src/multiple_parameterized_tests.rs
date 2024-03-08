use yare::parameterized;

#[parameterized(
    case1 = { 0 },
)]
fn test1(a: u32) {
    assert_eq!(a, 0)
}

#[parameterized(
    case1 = { 0 },
)]
fn test2(b: u32) {
    assert_eq!(b, 0)
}
