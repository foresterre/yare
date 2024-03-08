use yare::parameterized;

#[parameterized(
    case1 = { 0, 0 },
)]
fn test(a: u32, b: u32) {}

fn main() {}
