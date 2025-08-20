#![no_main]

use libfuzzer_sys::fuzz_target;
use bos_algo::{generate_distribution, Tier};

fuzz_target!(|data: &[u8]| {
    if data.len() < 16 {
        return;
    }

    // Parse BTC price from first 8 bytes
    let price_bytes: [u8; 8] = data[0..8].try_into().unwrap();
    let btc_price = u64::from_le_bytes(price_bytes);
    
    // Parse EUR cap from next 8 bytes
    let cap_bytes: [u8; 8] = data[8..16].try_into().unwrap();
    let eur_cap = u64::from_le_bytes(cap_bytes);
    
    // Limit ranges to reasonable values to avoid timeouts
    let btc_price = (btc_price % 1_000_000_00) + 1; // 1 to 1M EUR
    let eur_cap = eur_cap % 100_000_00; // 0 to 100K EUR
    
    // Try to generate distribution
    if let Ok(dist) = generate_distribution(btc_price, eur_cap) {
        // Verify invariants
        assert_eq!(dist.len(), 31_500, "Distribution must have exactly 31,500 bottles");
        
        // Count bottles per tier
        let mut tier_counts = [0usize; 6];
        let mut total_sats_f = 0u128;
        
        for bottle in &dist {
            let tier_idx = match bottle.tier {
                Tier::A => 0,
                Tier::B => 1,
                Tier::C => 2,
                Tier::D => 3,
                Tier::E => 4,
                Tier::F => 5,
            };
            tier_counts[tier_idx] += 1;
            
            // Verify satoshi amounts
            match bottle.tier {
                Tier::A => assert_eq!(bottle.sats, 1_000_000),
                Tier::B => assert_eq!(bottle.sats, 100_000),
                Tier::C => assert_eq!(bottle.sats, 10_000),
                Tier::D => assert_eq!(bottle.sats, 2_100),
                Tier::E => assert_eq!(bottle.sats, 1_000),
                Tier::F => {
                    assert!(bottle.sats >= 21 && bottle.sats <= 500,
                           "Tier F sats must be between 21 and 500");
                    total_sats_f += bottle.sats as u128;
                }
            }
            
            // All bottles should start unclaimed
            assert!(!bottle.claimed, "Bottles should start unclaimed");
        }
        
        // Verify tier counts
        assert_eq!(tier_counts[0], 1, "Tier A must have exactly 1 bottle");
        assert_eq!(tier_counts[1], 10, "Tier B must have exactly 10 bottles");
        assert_eq!(tier_counts[2], 100, "Tier C must have exactly 100 bottles");
        assert_eq!(tier_counts[3], 1_000, "Tier D must have exactly 1,000 bottles");
        assert_eq!(tier_counts[4], 2_000, "Tier E must have exactly 2,000 bottles");
        assert_eq!(tier_counts[5], 28_389, "Tier F must have exactly 28,389 bottles");
        
        // Verify cap is respected if set
        if eur_cap > 0 {
            let total_eur_cents = total_sats_f * btc_price as u128 / 100_000_000;
            assert!(total_eur_cents <= eur_cap as u128,
                   "Total EUR value of Tier F must not exceed cap");
        }
    }
});