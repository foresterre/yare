use yare::parameterized;

#[parameterized(
    case1 = { 0 },
)]
#[should_panic]
fn test1(_a: u32) {
    panic!("it should!");
}
