use yare::parameterized;

#[parameterized(two = { 2, 4 }, six = { 6, 12 }, eight = { 8, 16 })]
fn test_times2(input: i32, output: i32) {
    let times2 = |receiver: i32| receiver * 2;

    assert_eq!(times2(input), output);
}
