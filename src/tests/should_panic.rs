use yare::parameterized;

#[parameterized(none = { None })]
#[should_panic]
#[allow(clippy::unnecessary_literal_unwrap)]
fn numbers(input: Option<()>) {
    input.unwrap()
}
