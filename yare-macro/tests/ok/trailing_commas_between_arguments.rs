use yare_macro::parameterized;

#[parameterized(
    case1 = { 0, 1, },
    case2 = { 0, 1, },
)]
fn test(some: u32, more: u32) {}

fn main() {}
