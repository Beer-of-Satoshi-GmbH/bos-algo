use clap::Parser;
use std::collections::HashMap;
use bos_algo::{generate_distribution, Bottle, Tier};
use rand::seq::SliceRandom;
use rand::rng as thread_rng;

#[derive(Parser, Debug)]
#[command(name = "BOS Extended Simulation")]
#[command(author = "You")]
#[command(version = "1.0")]
#[command(about = "Generate 31,500-bottle distribution and claim step by step, tracking tier stats.")]
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

    let dist = match generate_distribution(args.price_eur_cents, args.cap_eur_cents) {
        Ok(d) => d,
        Err(err) => {
            eprintln!("Error generating distribution: {:?}", err);
            return;
        }
    };
    println!("Generated distribution of {} bottles.\n", dist.len());

    let mut session_dist: Vec<Bottle> = dist
        .into_iter()
        .map(|b| Bottle {
            tier: b.tier,
            sats: b.sats,
            claimed: false,
        })
        .collect();

    let mut tier_stats: HashMap<Tier, TierStats> = HashMap::new();
    /
    for b in &session_dist {
        let s = tier_stats.entry(b.tier).or_default();
        s.in_tier += 1;
    }

    fn print_tier_table(
        tier_stats: &HashMap<Tier, TierStats>,
        price_eur_cents: u64,
    ) {
        let all_tiers = [Tier::A, Tier::B, Tier::C, Tier::D, Tier::E, Tier::F];

        let mut grand_claimed_bottles = 0;
        let mut grand_in_tier = 0;
        let mut grand_sats_claimed: u128 = 0;

        println!(
            "{:25} | {:>7} | {:>7} | {:>9} | {:>12}",
            "Tier", "In Tier", "Claimed", "Unclaimed", "Sats Claimed"
        );
        println!("{:-<78}", "-");

        for &t in &all_tiers {
            if let Some(st) = tier_stats.get(&t) {
                let unclaimed = st.in_tier - st.claimed;
                grand_claimed_bottles += st.claimed;
                grand_in_tier += st.in_tier;
                grand_sats_claimed += st.sats_claimed;
                // Prepare a user-friendly name or label
                let tier_label = match t {
                    Tier::A => "Tier A (1M sats)",
                    Tier::B => "Tier B (100k sats)",
                    Tier::C => "Tier C (10k sats)",
                    Tier::D => "Tier D (2,100 sats)",
                    Tier::E => "Tier E (1,000 sats)",
                    Tier::F => "Tier F (21–500 random, cap)",
                };
                println!(
                    "{:25} | {:>7} | {:>7} | {:>9} | {:>12}",
                    tier_label,
                    st.in_tier,
                    st.claimed,
                    unclaimed,
                    st.sats_claimed
                );
            }
        }

        println!();
        println!(
            "Total Bottles: {}   Claimed so far: {}   Remaining unclaimed: {}",
            grand_in_tier,
            grand_claimed_bottles,
            grand_in_tier - grand_claimed_bottles
        );

        let sats_to_eur = price_eur_cents as f64 / 100_000_000.0;
        let total_eur_claimed = grand_sats_claimed as f64 * sats_to_eur;
        println!(
            "\nTotal Sats Claimed so far: {} sats   (~{:.2} EUR)\n",
            grand_sats_claimed,
            total_eur_claimed
        );
    }

    fn claim_bottles(
        session_dist: &mut [Bottle],
        tier_stats: &mut HashMap<Tier, TierStats>,
        how_many: usize,
    ) {
        let mut indices: Vec<usize> = session_dist
            .iter()
            .enumerate()
            .filter(|(_, b)| !b.claimed)
            .map(|(i, _)| i)
            .collect();

        indices.shuffle(&mut thread_rng());

        let actual = how_many.min(indices.len());
        let chosen = &indices[..actual];

        for &idx in chosen {
            if !session_dist[idx].claimed {
                session_dist[idx].claimed = true;
                let t = session_dist[idx].tier;
                let s = session_dist[idx].sats as u128;
                let ts = tier_stats.get_mut(&t).unwrap();
                ts.claimed += 1;
                ts.sats_claimed += s;
            }
        }
    }

    println!("Initial (no bottles claimed yet):");
    print_tier_table(&tier_stats, args.price_eur_cents);

    for step_idx in 0..args.simulate_steps {
        claim_bottles(&mut session_dist, &mut tier_stats, args.claim_step);
        println!(
            "After claiming {} bottles in step {}:",
            args.claim_step, step_idx + 1
        );
        print_tier_table(&tier_stats, args.price_eur_cents);
    }
}
