# bos‑algo

> **Beer of Satoshi** – secure, reproducible prize‑distribution logic  
> • Rust 2024 edition • no `unsafe` • fuzz‑tested • integer‑only money maths
---

## What it does

`bos‑algo` builds the **31,500‑bottle reward roster** for the Beer of Satoshi promotion.

| Tier | Bottles | Satoshi bonus per bottle                                            |
|------|---------|---------------------------------------------------------------------|
| A    | 1       | 1,000,000 sat                                                       |
| B    | 10      | 100,000 sat                                                         |
| C    | 100     | 10,000 sat                                                          |
| D    | 1,000   | 2,100 sat                                                           |
| E    | 2,000   | 1,000 sat                                                           |
| F    | 28,389  | **uniform 21–500 sat** <br> total never exceeds an optional EUR cap |

### Security & fairness highlights

- integer‑only conversions (no floats)
- cryptographically secure RNG (`rand` 0.9+)
- per‑bottle uniform draw that respects the global cap
- Fisher–Yates shuffle for unpredictable ordering
- zero `unsafe` code (`#![forbid(unsafe_code)]`)
- property‑based + fuzz tests for edge‑cases
- **Clippy** passes with `-D warnings` on MSRV 1.77

---

## Install

```bash
cargo add bos-algo
````

*(Or clone the repo and reference it via `cargo add --git <repo-url>`.)*

Requires **Rust 1.77+** and the **2024 edition**.

---

## Quick start

```rust
use bos_algo::generate_distribution;
/* Example values (09 Jun 2025):
   1 BTC ≈ €96,496.00 => 9,649,600 cents
   Cap   ≈ €10,000.00 => 1,000,000 cents */
let btc_price_cents = 9_649_600;
let eur_cap_cents   = 1_000_000;
let bottles = generate_distribution(btc_price_cents, eur_cap_cents).unwrap();
println!("generated {}", bottles.len());    // 31,500
println!("first bottle {:?}", bottles[0]); // random tier
```

* Pass `eur_cap_cents = 0` for “no cap.”
* If the cap is too small to allocate the **21 sat minimum** to every Tier F bottle, `generate_distribution` returns
  `Err(GenError::CapTooLow)`.

---

## Run the full test suite

```bash
git clone https://github.com/Beer-of-Satoshi-GmbH/bos-algo
cd bos-algo
cargo test            # unit + property tests
cargo clippy -- -D warnings   # must emit zero warnings
```

Property tests (`prop_invariants`) fuzz thousands of BTC‑price / EUR‑cap pairs and assert every invariant.
---

## CLI Simulation

We provide an example CLI tool (in `src/bin/sim.rs`) that:

* **Generates the distribution** once,
* **Claims bottles in steps** (e.g., 50 bottles each step, for 5 steps),
* **Prints a table** with Tier statistics (In Tier, Claimed, Unclaimed, Sats Claimed, etc.)
  To run the simulation:

1. **No cap**, 5 claiming steps, each claims 50 bottles:
   ```bash
   cargo run --bin sim -- \
     --price-eur-cents 9649600 \
     --cap-eur-cents 0 \
     --simulate-steps 5 \
     --claim-step 50
   ```
2. **Fetch price** from CoinGecko (pass 0 for `price-eur-cents`):
   ```bash
   cargo run --bin sim -- \
     --price-eur-cents 0 \
     --cap-eur-cents 1000000 \
     --simulate-steps 3 \
     --claim-step 100
   ```

---

## API

```rust
pub enum Tier {
    A,
    B,
    C,
    D,
    E,
    F
}
pub struct Bottle {
    pub tier:    Tier,
    pub sats:    u32,
    pub claimed: bool,
}
pub enum GenError {
    InvalidPrice,
    CapTooLow
}
pub fn generate_distribution(
    btc_price_eur_cents: u64,
    eur_cap_cents:       u64, // 0 ⇒ unlimited
) -> Result<Vec<Bottle>, GenError>
```

---

## Clippy linter

```bash
  $ cargo clippy -- -D warnings
```

---

## Audit checklist

| Item                      | Status                 |
|---------------------------|------------------------|
| Money handled as integers | ✅                      |
| RNG source                | `rand::thread_rng()` (CSPRNG) |
| `unsafe` code             | **None**               |
| Fuzz / property tests     | ✅ (`proptest`)         |
| MSRV                      | 1.77                   |
| License                   | MIT OR Apache‑2.0      |

---

## Contact

| Channel     | Handle                                            |
|-------------|---------------------------------------------------|
| Website     | [beerofsatoshi.com](https://beerofsatoshi.com/)   |
| Email       | [dev@beersatoshi.com](mailto:dev@beersatoshi.com) |
| X / Twitter | [@BeerOfSatoshi](https://x.com/BeerOfSatoshi)     |

Feel free to open an issue or pull request, or reach out on any of the above channels for questions, audits, or integration help.
---

## Development

### Quick Start with Make

```bash
make help         # Show all available commands
make test         # Run all tests
make bench        # Run benchmarks
make coverage     # Generate coverage report
make audit        # Security audit
make all          # Run fmt, lint, test, and audit
```

### CI/CD

This project uses GitHub Actions for continuous integration:
- Multi-platform testing (Linux, macOS, Windows)
- Multiple Rust versions (stable, beta, nightly, MSRV)
- Security auditing with cargo-audit
- Code coverage with codecov
- Automated dependency updates with Dependabot
- Fuzz testing and benchmarking

### Security

See [SECURITY.md](SECURITY.md) for our security policy and how to report vulnerabilities.

### Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for contribution guidelines.

## Upcoming features / ToDos

- [x] **CI/CD**: GitHub Actions integration
- [ ] **SIM**: Fetch BTC price from CoinGecko
- [ ] **UI**: Web UI for distribution simulation

## License

Licensed under **MIT**
