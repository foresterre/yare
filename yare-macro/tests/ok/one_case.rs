use yare_macro::parameterized;

#[parameterized(
    case1 = { 0 },
)]
#[should_panic]
fn test(value: u32) {}

fn main() {}
