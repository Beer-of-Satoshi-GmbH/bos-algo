# bos‑algo

> **Beer of Satoshi** – secure, reproducible prize‑distribution logic  
> • Rust 2024 edition • no `unsafe` • fuzz‑tested • integer‑only money maths • Clippy‑clean

---

## What it does

`bos‑algo` produces the **31 500‑bottle reward roster** for the Beer of Satoshi promotion.

| Tier | Bottles | Satoshi bonus per bottle |
|------|---------|--------------------------|
| A    | 1       | 1 000 000 sat |
| B    | 10      | 100 000 sat   |
| C    | 100     | 10 000 sat    |
| D    | 1 000   | 2 100 sat     |
| E    | 2 000   | 1 000 sat     |
| F    | 28 389  | **uniform 21 – 500 sat**<br>total never exceeds an optional EUR cap |

---

### Security & fairness highlights

- integer‑only conversions (no floats)
- cryptographically secure RNG (`rand` 0.9+ → `random_range`)
- per‑bottle uniform draw that respects the global cap
- Fisher–Yates shuffle for unpredictable ordering
- zero **`unsafe`** (`#![forbid(unsafe_code)]`)
- property‑based + fuzz tests for edge cases
- **Clippy** passes with `-D warnings` on MSRV 1.77

---

## Install

```bash
cargo add bos-algo
````

*(Or clone the repo and reference it via `cargo add --git <repo‑url>`.)*
Requires **Rust 1.77+** (Edition 2024).

---

## Quick start

```rust
use bos_algo::{generate_distribution, GenError};

fn main() -> Result<(), GenError> {
    /* Example values (09 Jun 2025):
       1 BTC ≈ €96 496.00 ⇒ 9_649_600 cents
       Cap   ≈ €10 000.00  ⇒ 1_000_000 cents */
    let btc_price_cents = 9_649_600;
    let eur_cap_cents   = 1_000_000;     // 0 = no cap

    let bottles = generate_distribution(btc_price_cents, eur_cap_cents)?;

    println!("generated {} bottles", bottles.len()); // 31 500
    println!("first bottle {:?}", bottles[0]);        // random tier
    Ok(())
}
```

If the cap is smaller than **28 389 × 21 sat** the function returns
`Err(GenError::CapTooLow)`.

---

## Run the full test suite

```bash
git clone https://github.com/Beer-of-Satoshi-GmbH/bos-algo
cd bos-algo
cargo test            # unit + property tests
cargo clippy -- -D warnings   # must emit zero warnings
```

---

## CLI simulation

The demo CLI (`src/bin/sim.rs`) lets you **claim bottles in steps** and
prints a live tier table.

### Example 1 — unlimited cap

```bash
cargo run --bin sim -- \
  --price-eur-cents 9649600 \
  --cap-eur-cents   0 \
  --simulate-steps  5 \
  --claim-step      50
```

### Example 2 — fetch price, 10 000 € cap

```bash
cargo run --bin sim -- \
  --price-eur-cents 0 \
  --cap-eur-cents   1000000 \
  --simulate-steps  3 \
  --claim-step      100
```

The tool fetches the BTC/EUR spot price from CoinGecko when
`--price-eur-cents 0` is given.

---

## Public API

```rust
pub enum Tier { A, B, C, D, E, F }

pub struct Bottle {
    pub tier:    Tier,
    pub sats:    u32,
    pub claimed: bool,
}

pub enum GenError { InvalidPrice, CapTooLow }

pub fn generate_distribution(
    btc_price_eur_cents: u64,
    eur_cap_cents:       u64,   // 0 ⇒ unlimited
) -> Result<Vec<Bottle>, GenError>;
```

---

## Clippy linter

```bash
  $ cargo clippy -- -D warnings
```


---

## Audit checklist

| Item                         | Status                 |
| ---------------------------- | ---------------------- |
| Money handled as integers    | ✅                      |
| RNG source                   | `rand::rng()` (CSPRNG) |
| `unsafe` code                | **None**               |
| Clippy clean (`-D warnings`) | ✅                      |
| Fuzz / property tests        | ✅ (`proptest`)         |
| MSRV                         | 1.77                   |
| Licence                      | MIT OR Apache‑2.0      |

---

## Contact

| Channel   | Handle                                                   |
| --------- | -------------------------------------------------------- |
| Website   | [https://beerofsatoshi.com/](https://beerofsatoshi.com/) |
| Email     | [dev@beersatoshi.com](mailto:dev@beersatoshi.com)        |
| X/Twitter | [**@BeerOfSatoshi**](https://x.com/BeerOfSatoshi)        |

Feel free to open an issue or PR, or reach out for questions, audits, or integration help.

---

## Roadmap / To‑dos

* [ ] **CI/CD**: build, test, Clippy gate
* [x] **CLI**: fetch BTC price from CoinGecko
* [ ] **Web UI**: browser‑based distribution simulator

---

## Licence

Licensed under **MIT**
