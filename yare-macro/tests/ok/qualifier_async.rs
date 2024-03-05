use yare_macro::parameterized;

#[parameterized(
    case1 = { 0 },
)]
async fn test(a: u32) {}

fn main() {}
