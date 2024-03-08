use yare::parameterized;

#[parameterized(
    case1 = { 0 },
)]
fn test(value: u32) {
    assert_eq!(value, 0);
}
