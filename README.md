# bos‑algo

> **Beer of Satoshi** – secure, reproducible prize‑distribution logic  
> • Rust 2024 edition • no `unsafe` • fuzz‑tested • integer‑only money maths

---

##  What it does

`bos‑algo` builds the **31  500‑bottle reward roster** for the Beer of Satoshi
promotion.

| Tier | Bottles | Satoshi bonus _per_ bottle |
|------|---------|---------------------------|
| A    | 1       | 1 000 000 sat |
| B    | 10      | 100 000 sat   |
| C    | 100     | 10 000 sat    |
| D    | 1 000   | 2 100 sat     |
| E    | 2 000   | 1 000 sat     |
| F    | 28 389  | **uniform 21 – 500 sat**<br>total never exceeds an optional EUR cap |

Security & fairness guarantees:

* **integer‑only** conversions (no floats)  
* **cryptographically secure RNG** (`rand 0.9`)  
* per‑bottle uniform draw that always respects the global cap  
* final Fisher–Yates shuffle for unpredictable ordering  
* zero `unsafe` code (`#![forbid(unsafe_code)]`)  
* property tests (₊ fuzz) covering edge‑cases

---

##  Install

```bash
cargo add bos-algo # or `cargo add --git <repo>`
````

Requires **Rust 1.77 +** (Edition 2024).

---

##  Quick start

```rust
use bos_algo::generate_distribution;

/*  Example values  (09 Jun 2025):
    1 BTC ≈ €96 496.00  → 9 649 600 cents
    Cap   ≈ €10 000.00  → 1 000 000 cents                               */
let btc_price_cents = 9_649_600;   // accurate to 0.01 €
let eur_cap_cents   = 1_000_000;   // €10 000 cap

let bottles = generate_distribution(btc_price_cents, eur_cap_cents).unwrap();

println!("generated {}", bottles.len());    // 31 500
println!("first bottle {:?}", bottles[0]);  // random tier
```

*Pass `eur_cap_cents = 0` for “no cap”.*
If the cap is too small to allocate the 21 sat minimum to every Tier F bottle,
`generate_distribution` returns `Err(GenError::CapTooLow)`.

---

##  Run the full test‑suite

```bash
git clone https://github.com/Beer-of-Satoshi-GmbH/bos-algo
cd bos‑algo
cargo test
```

Property tests (`prop_invariants`) fuzz thousands of BTC‑price / EUR‑cap pairs
and assert every invariant.

---

##  API

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
) -> Result<Vec<Bottle>, GenError>
```

---

##  Audit checklist

| Item                      | Status                 |
| ------------------------- | ---------------------- |
| Money handled as integers | ✅                      |
| RNG source                | `rand::rng()` (CSPRNG) |
| `unsafe` code             | **None**               |
| Fuzz / property tests     | ✅ (`proptest`)         |
| MSRV                      | 1.77                   |
| Licence                   | MIT OR Apache‑2.0      |

---

##  Contact

| Channel     | Handle                                                                        |
|-------------|-------------------------------------------------------------------------------|
| Website     | [beerofsatoshi.com](https://beerofsatoshi.com/)                               |
| Email       | [dev@beersatoshi.com](mailto:dev@beersatoshi.com)                             |
| X / Twitter | [@BeerOfSatoshi](https://x.com/BeerOfSatoshi)                                 |


Feel free to open an issue or pull‑request, or reach out on any of the above
channels for questions, audits, or integration help.

---

##  Licence

Licensed under **MIT**
