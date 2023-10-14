use yare_macro::parameterized;

#[parameterized(
    case1 = { 0 },
)]
fn test(a: u32) {}

fn main() {}
