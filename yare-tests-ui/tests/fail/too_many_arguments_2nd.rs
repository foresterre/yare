use yare::parameterized;

#[parameterized(
    case = { 0, 1 },
    case2 = { 0, 1, 2 },
)]
fn test(some: u32, more: u32) {}

fn main() {}
