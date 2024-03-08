use yare::parameterized;

#[parameterized(
    case1 = { 0, 0, },
    case2 = { 1, 1, },
)]
fn test(some: u32, more: u32) {
    assert_eq!(some, more)
}
