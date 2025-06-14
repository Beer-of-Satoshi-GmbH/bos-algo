use bos_algo::{Bottle, Tier, generate_distribution};
use proptest::prelude::*;

const TOTAL: usize = 31_500;
const F_COUNT: usize = 28_389;
const MIN_F: u32 = 21;
const MAX_F: u32 = 500;
const ONE_BTC_SATS: u128 = 100_000_000;

fn fixed_counts_ok(dist: &[Bottle]) -> bool {
    let mut counts = [0usize; 6];
    for b in dist {
        counts[b.tier as usize] += 1;
    }
    counts == [1, 10, 100, 1_000, 2_000, F_COUNT]
}

#[test]
fn rejects_zero_price() {
    assert!(matches!(
        generate_distribution(0, 0).unwrap_err(),
        bos_algo::GenError::InvalidPrice
    ));
}

#[test]
fn rejects_cap_too_low() {
    let price = 96_496_00;
    let min_cap_cents = (F_COUNT as u128 * MIN_F as u128 * price as u128) / ONE_BTC_SATS;
    let too_low = min_cap_cents as u64 - 1;
    assert!(matches!(
        generate_distribution(price, too_low).unwrap_err(),
        bos_algo::GenError::CapTooLow
    ));
}

#[test]
fn unlimited_cap_invariants_hold() {
    let price = 96_496_00;
    let dist = generate_distribution(price, 0).unwrap();
    assert_eq!(dist.len(), TOTAL);
    assert!(fixed_counts_ok(&dist));
    assert!(
        dist.iter()
            .filter(|b| b.tier == Tier::F)
            .all(|b| (MIN_F..=MAX_F).contains(&b.sats))
    );
}

#[test]
fn cap_respected_exact() {
    let price = 96_496_00;
    let cap = 10_000_00;
    let dist = generate_distribution(price, cap).unwrap();

    let total_f_sats: u128 = dist
        .iter()
        .filter(|b| b.tier == Tier::F)
        .map(|b| u128::from(b.sats))
        .sum();

    let total_eur_cents = total_f_sats * price as u128 / ONE_BTC_SATS;
    assert!(total_eur_cents <= cap as u128);
}

proptest! {
    #[test]
    fn prop_invariants(
        price in 1u64..=200_000_00,
        spare in 0u64..=20_000_00
    ) {
        let min_cap_cents =
            (F_COUNT as u128 * MIN_F as u128 * price as u128) / ONE_BTC_SATS;
        let cap = min_cap_cents as u64 + spare;

        let dist = generate_distribution(price, cap).unwrap();

        prop_assert_eq!(dist.len(), TOTAL);
        prop_assert!(fixed_counts_ok(&dist));

        let mut min_seen = u32::MAX;
        let mut max_seen = 0;
        let mut total_f_sats: u128 = 0;

        for b in &dist {
            if b.tier == Tier::F {
                min_seen = min_seen.min(b.sats);
                max_seen = max_seen.max(b.sats);
                total_f_sats += u128::from(b.sats);
            }
        }

        prop_assert!(min_seen >= MIN_F);
        prop_assert!(max_seen <= MAX_F);

        let eur_cents = total_f_sats * price as u128 / ONE_BTC_SATS;
        prop_assert!(eur_cents <= cap as u128);
    }
}
