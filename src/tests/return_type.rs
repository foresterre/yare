use yare::parameterized;

#[parameterized(
    ok = { Ok(0) },
    // err = {  Err("noes!".to_string()) }, <-- enabling this would result in a failed test, since it doesn't terminate with a ErrorCode::Success, via the Termination trait
)]
fn test(value: Result<u32, String>) -> Result<(), String> {
    value.map(|_| ())
}

#[parameterized(
    ok = { Ok(0) },
)]
fn try_operator(value: Result<u32, String>) -> Result<(), String> {
    let _ = value?;

    Ok(())
}
