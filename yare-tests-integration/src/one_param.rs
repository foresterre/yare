use yare::parameterized;

#[parameterized(
    case1 = { 0 },
)]
fn test(a: u32) {
    assert_eq!(a, 0);
}
