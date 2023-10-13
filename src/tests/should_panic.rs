use yare::parameterized;

#[parameterized(none = { None })]
#[should_panic]
fn numbers(input: Option<()>) {
    input.unwrap()
}
