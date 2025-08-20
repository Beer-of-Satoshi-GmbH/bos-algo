#![no_main]

use libfuzzer_sys::fuzz_target;
use bos_algo::{generate_distribution, GenError};

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
    
    // Test edge cases
    
    // 1. Zero price should always error
    if btc_price == 0 {
        assert!(matches!(
            generate_distribution(0, eur_cap),
            Err(GenError::InvalidPrice)
        ));
        return;
    }
    
    // 2. Test minimum viable cap
    if btc_price > 0 && btc_price < u64::MAX / 28_389 {
        let min_cap = (28_389u128 * 21 * btc_price as u128 / 100_000_000) as u64;
        
        if eur_cap > 0 && eur_cap < min_cap {
            // Cap too low should error
            assert!(matches!(
                generate_distribution(btc_price, eur_cap),
                Err(GenError::CapTooLow)
            ));
        } else if eur_cap >= min_cap || eur_cap == 0 {
            // Should succeed
            let result = generate_distribution(btc_price, eur_cap);
            assert!(result.is_ok(), "Distribution should succeed with valid inputs");
        }
    }
    
    // 3. Test extreme values (with limits to prevent overflow)
    if btc_price == u64::MAX {
        // Very high BTC price with low cap should fail
        if eur_cap > 0 && eur_cap < 1_000_000 {
            assert!(matches!(
                generate_distribution(btc_price, eur_cap),
                Err(GenError::CapTooLow)
            ));
        }
    }
    
    // 4. Test that no-cap (0) always works with valid price
    if btc_price > 0 && btc_price < u64::MAX / 100 {
        let result = generate_distribution(btc_price, 0);
        assert!(result.is_ok(), "No-cap distribution should always work with valid price");
        
        if let Ok(dist) = result {
            // Verify all Tier F bottles have maximum range available
            for bottle in dist {
                if bottle.tier == bos_algo::Tier::F {
                    assert!(bottle.sats >= 21 && bottle.sats <= 500);
                }
            }
        }
    }
});