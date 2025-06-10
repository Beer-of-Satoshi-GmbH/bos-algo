//! CLI demo – generate once, then claim in steps and print tier table.

use clap::Parser;
use rand::{rng, seq::SliceRandom};
use serde::Deserialize;
use reqwest::blocking;
use std::collections::HashMap;

use bos_algo::{generate_distribution, Bottle, Tier};

#[derive(Parser, Debug)]
#[command(name = "bos‑sim")]
#[command(author = "Beer of Satoshi")]
#[command(version = "0.1")]
#[command(about = "Step‑wise bottle claiming simulation")]
struct Args {
    #[arg(long, default_value = "0")]
    price_eur_cents: u64,

    #[arg(long, default_value = "0")]
    cap_eur_cents: u64,

    #[arg(long, default_value = "5")]
    simulate_steps: usize,

    #[arg(long, default_value = "50")]
    claim_step: usize,
}

#[derive(Deserialize)]
struct Gecko { bitcoin: Price }
#[derive(Deserialize)]
struct Price  { eur: f64 }

fn fetch_btc_price_eur_cents() -> Result<u64, Box<dyn std::error::Error>> {
    let url = "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=eur";
    let g: Gecko = blocking::get(url)?.json()?;
    Ok((g.bitcoin.eur * 100.0).round() as u64)
}

#[derive(Default)]
struct TierStats {
    in_tier:      u16,
    claimed:      u16,
    sats_claimed: u64,
}

fn main() {
    let mut args = Args::parse();

    if args.price_eur_cents == 0 {
        match fetch_btc_price_eur_cents() {
            Ok(p) => {
                println!("Fetched BTC price: {:.2} €", p as f64 / 100.0);
                args.price_eur_cents = p;
            }
            Err(e) => {
                eprintln!("Price fetch failed: {e}. Using 9 649 600 c.");
                args.price_eur_cents = 9_649_600;
            }
        }
    }

    let dist = match generate_distribution(args.price_eur_cents, args.cap_eur_cents) {
        Ok(d)  => d,
        Err(e) => {
            eprintln!("Generation error: {e:?}");
            return;
        }
    };
    println!("Distribution generated: {} bottles\n", dist.len());

    let mut session_dist: Vec<Bottle> = dist
        .into_iter()
        .map(|b| Bottle { claimed: false, ..b })
        .collect();

    let mut stats: HashMap<Tier, TierStats> = HashMap::new();
    for bottle in &session_dist {
        stats.entry(bottle.tier).or_default().in_tier += 1;
    }

    let mut rng = rng();

    fn print_table(stats: &HashMap<Tier, TierStats>, price_eur_cents: u64) {
        const TIERS: [Tier; 6] = [Tier::A, Tier::B, Tier::C, Tier::D, Tier::E, Tier::F];

        let mut total_in = 0u32;
        let mut total_cl = 0u32;
        let mut sats_sum = 0u64;

        println!("{:25} | {:>7} | {:>7} | {:>9} | {:>13}",
                 "Tier", "In Tier", "Claimed", "Unclaimed", "Sats Claimed");
        println!("{:-<81}", "-");

        for &t in &TIERS {
            if let Some(st) = stats.get(&t) {
                let un = st.in_tier - st.claimed;
                total_in += st.in_tier as u32;
                total_cl += st.claimed as u32;
                sats_sum += st.sats_claimed;

                let label = match t {
                    Tier::A=>"Tier A (1M)",   Tier::B=>"Tier B (100k)",
                    Tier::C=>"Tier C (10k)",  Tier::D=>"Tier D (2 100)",
                    Tier::E=>"Tier E (1 000)",Tier::F=>"Tier F (21–500 cap)",
                };
                println!("{:25} | {:>7} | {:>7} | {:>9} | {:>13}",
                         label, st.in_tier, st.claimed, un, st.sats_claimed);
            }
        }

        let remaining = total_in - total_cl;
        let eur = sats_sum as f64 * (price_eur_cents as f64 / 100_000_000.0);

        println!("\nTotal bottles: {total_in}   Claimed: {total_cl}   Remaining: {remaining}");
        println!("Total sats claimed: {sats_sum} ≈ {:.2} €\n", eur);
    }

    fn claim_n(
        how_many: usize,
        dist: &mut [Bottle],
        stats: &mut HashMap<Tier, TierStats>,
        rng: &mut impl rand::Rng,
    ) {
        let mut unclaimed: Vec<usize> = dist.iter()
            .enumerate()
            .filter(|(_, b)| !b.claimed)
            .map(|(i, _)| i)
            .collect();

        unclaimed.shuffle(rng);
        let pick = how_many.min(unclaimed.len());
        for &idx in &unclaimed[..pick] {
            let b = &mut dist[idx];
            b.claimed = true;
            let s = stats.get_mut(&b.tier).unwrap();
            s.claimed      += 1;
            s.sats_claimed += b.sats as u64;
        }
    }

    println!("Initial state:");
    print_table(&stats, args.price_eur_cents);

    for step in 1..=args.simulate_steps {
        claim_n(args.claim_step, &mut session_dist, &mut stats, &mut rng);
        println!("After claiming {} bottles (step {step}/{})",
            args.claim_step, args.simulate_steps);
        print_table(&stats, args.price_eur_cents);
    }
}
