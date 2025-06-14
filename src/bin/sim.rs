#![forbid(unsafe_code)]
#![warn(clippy::all, clippy::pedantic)]

use clap::Parser;
use rand::{rng, seq::SliceRandom};
use std::collections::HashMap;

use bos_algo::{Bottle, Tier, generate_distribution};

#[derive(Parser, Debug)]
#[command(name = "BOS Extended Simulation")]
#[command(author = "Beer of Satoshi")]
#[command(version = "1.0")]
#[command(
    about = "Generate the 31 500‑bottle distribution and step‑claim bottles, \
             tracking tier statistics."
)]
struct Cli {
    #[arg(long, default_value = "9649600")]
    price_eur_cents: u64,

    #[arg(long, default_value = "0")]
    cap_eur_cents: u64,

    #[arg(long, default_value = "5")]
    simulate_steps: usize,

    #[arg(long, default_value = "50")]
    claim_step: usize,
}

#[derive(Default)]
struct TierStats {
    in_tier: usize,
    claimed: usize,
    sats_claimed: u128,
}

fn main() {
    let args = Cli::parse();

    let dist =
        generate_distribution(args.price_eur_cents, args.cap_eur_cents).unwrap_or_else(|e| {
            eprintln!("Error generating distribution: {e:?}");
            std::process::exit(1);
        });

    println!("Generated distribution of {} bottles.\n", dist.len());

    let mut session_dist: Vec<Bottle> = dist
        .into_iter()
        .map(|b| Bottle {
            claimed: false,
            ..b
        })
        .collect();

    let mut tier_stats: HashMap<Tier, TierStats> = HashMap::new();
    for b in &session_dist {
        tier_stats.entry(b.tier).or_default().in_tier += 1;
    }

    print_tier_table(
        &tier_stats,
        args.price_eur_cents,
        "Initial (no bottles claimed yet)",
    );

    for step in 1..=args.simulate_steps {
        claim_bottles(&mut session_dist, &mut tier_stats, args.claim_step);
        let caption = format!("After claiming {} bottles in step {step}", args.claim_step);
        print_tier_table(&tier_stats, args.price_eur_cents, &caption);
    }
}

fn print_tier_table(stats: &HashMap<Tier, TierStats>, price_cents: u64, header: &str) {
    println!("\n{header}");
    println!(
        "{:25} | {:>7} | {:>7} | {:>9} | {:>12}",
        "Tier", "In", "Claimed", "Unclaimed", "Sat‑Claimed"
    );
    println!("{:-<78}", "-");

    let labels = [
        (Tier::A, "Tier A (1 000 000 sat)"),
        (Tier::B, "Tier B (100 000 sat)"),
        (Tier::C, "Tier C (10 000 sat)"),
        (Tier::D, "Tier D (2 100 sat)"),
        (Tier::E, "Tier E (1 000 sat)"),
        (Tier::F, "Tier F (21‑500 sat)"),
    ];

    let mut total_bottles = 0;
    let mut total_claimed = 0;
    let mut sats_total: u128 = 0;

    for (tier, label) in labels {
        if let Some(s) = stats.get(&tier) {
            let unclaimed = s.in_tier - s.claimed;
            total_bottles += s.in_tier;
            total_claimed += s.claimed;
            sats_total += s.sats_claimed;

            println!(
                "{:25} | {:>7} | {:>7} | {:>9} | {:>12}",
                label, s.in_tier, s.claimed, unclaimed, s.sats_claimed
            );
        }
    }

    let eur_total_cents: u128 = sats_total * u128::from(price_cents) / 100_000_000u128;

    println!(
        "\nTotals → bottles: {total_bottles}, claimed: {total_claimed}, remaining: {}",
        total_bottles - total_claimed
    );

    let euros = eur_total_cents / 100;
    let cents = eur_total_cents % 100;
    println!("Total sats claimed: {sats_total} ≈ €{euros}.{cents:02}");
}

fn claim_bottles(dist: &mut [Bottle], stats: &mut HashMap<Tier, TierStats>, how_many: usize) {
    let mut indices: Vec<_> = dist
        .iter()
        .enumerate()
        .filter_map(|(i, b)| (!b.claimed).then_some(i))
        .collect();

    let mut rng_local = rng();
    indices.shuffle(&mut rng_local);

    for &idx in indices.iter().take(how_many) {
        let bottle = &mut dist[idx];
        if !bottle.claimed {
            bottle.claimed = true;
            let entry = stats.get_mut(&bottle.tier).expect("tier present");
            entry.claimed += 1;
            entry.sats_claimed += u128::from(bottle.sats);
        }
    }
}
