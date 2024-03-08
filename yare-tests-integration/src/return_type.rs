use yare::parameterized;

#[parameterized(
    case1 = { Ok(0) },
)]
fn test(value: Result<u32, ()>) -> Result<(), ()> {
    let value = value?;
    assert_eq!(value, 0);

    Ok(())
}
