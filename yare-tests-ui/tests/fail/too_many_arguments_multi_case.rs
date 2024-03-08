use yare::parameterized;

#[parameterized(
    case1 = { 0, 1, 2 },
    case2 = { 0, 1, 2 },
)]
fn test(some: u32, more: u32) {}

fn main() {}
