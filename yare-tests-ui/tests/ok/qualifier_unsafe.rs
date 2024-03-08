use yare::parameterized;

#[parameterized(
    case1 = { 0 },
)]
unsafe fn test(a: u32) {}

fn main() {}
