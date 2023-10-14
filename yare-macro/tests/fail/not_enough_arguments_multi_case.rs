use yare_macro::parameterized;

#[parameterized(
    case1 = { 0 },
    case2 = { 0 },
)]
fn test(some: u32, more: u32) {}

fn main() {}
