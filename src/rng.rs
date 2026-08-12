/// Version-one deterministic PRNG used by game setup.
///
/// The algorithm is deliberately owned by the engine so a dependency upgrade
/// cannot silently make old seeds produce different replays.
#[derive(Clone, Debug)]
pub(crate) struct ReplayRng {
    state: u64,
}

impl ReplayRng {
    pub(crate) const fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut value = self.state;
        value = (value ^ (value >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        value = (value ^ (value >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        value ^ (value >> 31)
    }

    pub(crate) fn shuffle<T>(&mut self, values: &mut [T]) {
        for upper in (1..values.len()).rev() {
            let index = self.index_inclusive(upper);
            values.swap(upper, index);
        }
    }

    /// Samples a floating-point probability from a uniform 53-bit value.
    ///
    /// Every valid probability advances the generator exactly once, including
    /// the impossible and certain boundary cases. Keeping those cases on the
    /// same path makes the replay stream independent of likelihood shortcuts.
    #[allow(clippy::cast_precision_loss)]
    pub(crate) fn sample_probability(&mut self, likelihood: f64) -> bool {
        const UNIT_53: f64 = 1.0 / 9_007_199_254_740_992.0;
        assert!(
            (0.0..=1.0).contains(&likelihood),
            "likelihood must be finite and between 0.0 and 1.0"
        );
        let sample = (self.next_u64() >> 11) as f64 * UNIT_53;
        sample < likelihood
    }

    fn index_inclusive(&mut self, upper: usize) -> usize {
        let range = u64::try_from(upper)
            .expect("slice indexes fit in u64")
            .checked_add(1)
            .expect("slice length fits in u64");
        usize::try_from(self.uniform_below(range)).expect("result is at most a slice index")
    }

    /// Uniformly samples `[0, exclusive_upper)` by rejecting the short leading
    /// interval that would bias a `u64` reduced modulo the range.
    fn uniform_below(&mut self, exclusive_upper: u64) -> u64 {
        debug_assert!(exclusive_upper != 0);
        let range = exclusive_upper;
        let threshold = range.wrapping_neg() % range;
        loop {
            let value = self.next_u64();
            if value >= threshold {
                return value % range;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ReplayRng;

    #[test]
    fn shuffle_is_stable_for_a_known_seed() {
        let mut values = [0, 1, 2, 3, 4, 5, 6, 7];
        ReplayRng::new(42).shuffle(&mut values);
        assert_eq!(values, [3, 1, 6, 2, 4, 0, 7, 5]);
    }

    #[test]
    fn probability_sampling_is_stable_for_a_known_seed() {
        let mut rng = ReplayRng::new(0);
        let samples = std::array::from_fn::<_, 8, _>(|_| rng.sample_probability(0.9));
        assert_eq!(samples, [true, true, true, false, true, true, true, true]);
    }

    #[test]
    fn impossible_and_certain_probabilities_each_consume_a_draw() {
        let mut expected = ReplayRng::new(42);
        let _first = expected.next_u64();
        let state_after_one_draw = expected.state;
        let _second = expected.next_u64();
        let state_after_two_draws = expected.state;

        let mut impossible = ReplayRng::new(42);
        assert!(!impossible.sample_probability(0.0));
        assert_eq!(impossible.state, state_after_one_draw);

        let mut certain = ReplayRng::new(42);
        assert!(certain.sample_probability(1.0));
        assert_eq!(certain.state, state_after_one_draw);
        assert!(certain.sample_probability(1.0));
        assert_eq!(certain.state, state_after_two_draws);
    }

    #[test]
    #[should_panic(expected = "likelihood must be finite and between 0.0 and 1.0")]
    fn probability_sampling_rejects_nan() {
        let _ = ReplayRng::new(0).sample_probability(f64::NAN);
    }

    #[test]
    #[should_panic(expected = "likelihood must be finite and between 0.0 and 1.0")]
    fn probability_sampling_rejects_negative_values() {
        let _ = ReplayRng::new(0).sample_probability(-0.1);
    }

    #[test]
    #[should_panic(expected = "likelihood must be finite and between 0.0 and 1.0")]
    fn probability_sampling_rejects_values_above_one() {
        let _ = ReplayRng::new(0).sample_probability(1.1);
    }
}
