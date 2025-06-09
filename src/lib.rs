#![forbid(unsafe_code)]

use rand::{rng, Rng};
use rand::seq::SliceRandom;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Tier { A, B, C, D, E, F }

#[derive(Clone, Debug)]
pub struct Bottle {
    pub tier: Tier,
    pub sats: u32,
    pub claimed: bool,
}

#[derive(Debug, PartialEq)]
pub enum GenError { InvalidPrice, CapTooLow }

pub fn generate_distribution(
    btc_price_eur_cents: u64,
    eur_cap_cents: u64,
) -> Result<Vec<Bottle>, GenError> {
    if btc_price_eur_cents == 0 {
        return Err(GenError::InvalidPrice);
    }

    const TOTAL_BOTTLES: usize = 31_500;
    const MIN_F: u32 = 21;
    const MAX_F: u32 = 500;
    const ONE_BTC_SATS: u128 = 100_000_000;
    const FIXED: &[(Tier, usize, u32)] = &[
        (Tier::A,    1,    1_000_000),
        (Tier::B,   10,      100_000),
        (Tier::C,  100,       10_000),
        (Tier::D,1_000,        2_100),
        (Tier::E,2_000,        1_000),
    ];

    let mut dist = Vec::with_capacity(TOTAL_BOTTLES);
    let mut assigned = 0;
    for &(tier, count, sats) in FIXED {
        dist.extend(std::iter::repeat(Bottle { tier, sats, claimed: false }).take(count));
        assigned += count;
    }

    let tier_f_count = TOTAL_BOTTLES - assigned;
    let mut rng = rng();

    if tier_f_count == 0 {
        dist.shuffle(&mut rng);
        return Ok(dist);
    }

    let budget_sats = if eur_cap_cents == 0 {
        (MAX_F as u128) * (tier_f_count as u128)
    } else {
        (eur_cap_cents as u128).saturating_mul(ONE_BTC_SATS) / (btc_price_eur_cents as u128)
    };

    let min_possible = (MIN_F as u128) * (tier_f_count as u128);
    if budget_sats < min_possible {
        return Err(GenError::CapTooLow);
    }

    let mut remaining = tier_f_count as u32;
    let mut rem_budget = budget_sats;
    while remaining > 0 {
        let needed_for_rest = (MIN_F as u128) * ((remaining - 1) as u128);
        let mut feasible_max = (rem_budget.saturating_sub(needed_for_rest)) as u32;
        if feasible_max > MAX_F {
            feasible_max = MAX_F;
        }
        let val = rng.random_range(MIN_F..=feasible_max);
        dist.push(Bottle { tier: Tier::F, sats: val, claimed: false });
        rem_budget -= val as u128;
        remaining -= 1;
    }

    dist.shuffle(&mut rng);
    Ok(dist)
}
