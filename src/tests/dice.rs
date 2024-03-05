use yare::parameterized;

use std::sync::atomic::{AtomicU32, Ordering};

fn roll_dice(seed: &AtomicU32, pips: u32) -> u8 {
    let mut random = seed.load(Ordering::SeqCst);
    random ^= random << 13;
    random ^= random >> 17;
    random ^= random << 5;
    seed.store(random, Ordering::SeqCst);

    (random % pips + 1) as u8
}

#[parameterized(
    roll_d6 = {6, 1000, |outcome: u8| (1_u8..=6).contains(&outcome)},
    roll_d12 = {12, 1000, |outcome: u8| (1_u8..=12).contains(&outcome) }
)]
fn dicey(pips: u32, rolls: u32, test: fn(u8) -> bool) {
    let seed = AtomicU32::new(1);

    let rolls = (0..rolls)
        .map(|_| roll_dice(&seed, pips))
        .collect::<Vec<_>>();
    let unacceptable_outcome = rolls.iter().find(|&&outcome| !test(outcome));

    assert!(unacceptable_outcome.is_none());
}
