use yare::parameterized;

#[parameterized(
    zero_wait = { 0, 0 },
    show_paused = { 500, 0 },
)]
#[test_macro(tokio::test(start_paused = true))]
async fn test(wait: u64, time_elapsed: u128) {
    let start = std::time::Instant::now();
    tokio::time::sleep(tokio::time::Duration::from_millis(wait)).await;

    assert_eq!(time_elapsed, start.elapsed().as_millis());
}
