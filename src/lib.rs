#![forbid(unsafe_code)]
#![warn(clippy::all, clippy::pedantic)]

use rand::seq::SliceRandom;
use rand::{Rng, rng};
use std::convert::TryFrom;

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Tier {
    A,
    B,
    C,
    D,
    E,
    F,
}

#[derive(Clone, Debug)]
pub struct Bottle {
    pub tier: Tier,
    pub sats: u32,
    pub claimed: bool,
}

#[derive(Debug, PartialEq)]
pub enum GenError {
    InvalidPrice,
    CapTooLow,
}

const TOTAL_BOTTLES: usize = 31_500;
const MIN_F: u32 = 21;
const MAX_F: u32 = 500;
const ONE_BTC_SATS: u128 = 100_000_000;

const FIXED: &[(Tier, usize, u32)] = &[
    (Tier::A, 1, 1_000_000),
    (Tier::B, 10, 100_000),
    (Tier::C, 100, 10_000),
    (Tier::D, 1_000, 2_100),
    (Tier::E, 2_000, 1_000),
];

/// Build the full 31 500‑bottle distribution.
///
/// # Errors
///
/// * `GenError::InvalidPrice` – `btc_price_eur_cents` is 0.
/// * `GenError::CapTooLow` – `eur_cap_cents` cannot fund 21 sat for every Tier F bottle.
///
/// # Panics
///
/// Panics only if the constant `TOTAL_BOTTLES` is increased beyond `u32::MAX`,
/// because the Tier F bottle count is converted into `u32` with `expect`.
pub fn generate_distribution(
    btc_price_eur_cents: u64,
    eur_cap_cents: u64,
) -> Result<Vec<Bottle>, GenError> {
    if btc_price_eur_cents == 0 {
        return Err(GenError::InvalidPrice);
    }

    let mut dist = Vec::with_capacity(TOTAL_BOTTLES);
    for &(tier, count, sats) in FIXED {
        dist.extend(
            std::iter::repeat_with(|| Bottle {
                tier,
                sats,
                claimed: false,
            })
            .take(count),
        );
    }

    let tier_f_count: u32 =
        u32::try_from(TOTAL_BOTTLES - dist.len()).expect("≤ 31 500 fits in u32");
    let mut rng = rng();

    if tier_f_count == 0 {
        dist.shuffle(&mut rng);
        return Ok(dist);
    }

    let budget_sats: u128 = if eur_cap_cents == 0 {
        u128::from(MAX_F) * u128::from(tier_f_count)
    } else {
        u128::from(eur_cap_cents).saturating_mul(ONE_BTC_SATS) / u128::from(btc_price_eur_cents)
    };

    let min_possible = u128::from(MIN_F) * u128::from(tier_f_count);
    if budget_sats < min_possible {
        return Err(GenError::CapTooLow);
    }

    let mut remaining = tier_f_count;
    let mut rem_budget = budget_sats;

    while remaining > 0 {
        let needed_for_rest = u128::from(MIN_F) * u128::from(remaining - 1);

        let feasible_max_u128 = rem_budget
            .saturating_sub(needed_for_rest)
            .min(u128::from(MAX_F));

        let feasible_max: u32 = u32::try_from(feasible_max_u128).unwrap_or(MAX_F);

        let val = rng.random_range(MIN_F..=feasible_max);

        dist.push(Bottle {
            tier: Tier::F,
            sats: val,
            claimed: false,
        });

        rem_budget -= u128::from(val);
        remaining -= 1;
    }

    dist.shuffle(&mut rng);
    Ok(dist)
}
