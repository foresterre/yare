use yare_macro::parameterized;

#[parameterized(
    case1 = { 0 },
)]
unsafe fn test(a: u32) {}

fn main() {}
