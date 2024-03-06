use std::sync::atomic::{AtomicU32, Ordering};

fn roll_dice(seed: &AtomicU32) -> u8 {
    let mut random = seed.load(Ordering::SeqCst);
    random ^= random << 13;
    random ^= random >> 17;
    random ^= random << 5;
    seed.store(random, Ordering::SeqCst);

    (random % 6 + 1) as u8
}

#[cfg(test)]
mod tests {
    use super::*;
    use yare::parameterized;

    #[parameterized(
        seeding_randomness_with_two_dice_rolls = { roll_dice(&AtomicU32::new(0)),  {
            let from_half = roll_dice( & AtomicU32::new(u32::MAX / 2));
            let from_max = roll_dice( & AtomicU32::new(u32::MAX));

            u8::min(from_half, from_max)
        }}
    )]
    fn dicey(seed1: u8, seed2: u8) {
        let max = u8::max(seed1, seed2);
        let seed = AtomicU32::new(u32::from(max));

        let out_of_bounds_values = (0..1000)
            .map(|_| roll_dice(&seed))
            .find(|value| !(1..=6).contains(value));

        assert!(out_of_bounds_values.is_none());
    }
}
