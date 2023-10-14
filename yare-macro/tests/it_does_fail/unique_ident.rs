use yare_macro::parameterized;

// It actually does fail to compile, but idk why not for trybuild.

#[parameterized(
    case1 = { 0, 1, },
    case1 = { 0, 1, },
)]
fn test(some: u32, more: u32) {}

fn main() {}
