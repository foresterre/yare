use yare::parameterized;

#[parameterized(
    case1 = { 0 },
)]
#[test_macro(tokio::test)]
async fn test(a: u32) {
    assert_eq!(0, a);
}
