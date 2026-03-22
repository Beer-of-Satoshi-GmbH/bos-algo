---
name: Feature request
about: Add support for deterministic RNG to allow auditing of the distribution.
title: '[FEATURE] Implement RNG Injection for Auditability'
labels: enhancement, security, audit
assignees: ''

---

**Is your feature request related to a problem? Please describe.**
The `generate_distribution` function uses `rand::thread_rng()` internally. This prevents developers from reproducing a specific distribution for auditing or dispute resolution. For a promotion involving financial prizes, it is essential to be able to recreate the exact prize set from a recorded seed or RNG state.

**Describe the solution you'd like**
Expose a version of the generation function that accepts a generic RNG or a seed:
```rust
pub fn generate_distribution_with_rng<R: Rng>(
    btc_price_eur_cents: u64,
    eur_cap_cents: u64,
    rng: &mut R,
) -> Result<Vec<Bottle>, GenError>
```

**Describe alternatives you've considered**
None. RNG injection is the industry standard for reproducible random algorithms.

**Example usage**
```rust
use rand_chacha::ChaCha20Rng;
use rand::SeedableRng;

let mut rng = ChaCha20Rng::seed_from_u64(12345);
let dist = generate_distribution_with_rng(price, cap, &mut rng).unwrap();
```

**Additional context**
Without this, any audit of the distribution requires full capture of the generated list at runtime, which is more prone to loss or manipulation.
