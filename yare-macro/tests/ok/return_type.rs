use yare_macro::parameterized;

#[parameterized(
    case1 = { 0 },
)]
fn test(value: u32) -> Result<(), ()> {
    Ok(())
}

fn main() {}
