---
name: Feature request
about: Integrate CoinGecko API into the sim binary.
title: '[FEATURE] SIM: Fetch BTC Price from CoinGecko'
labels: enhancement, sim
assignees: ''

---

**Is your feature request related to a problem? Please describe.**
The `sim.rs` binary currently requires a manual `--price-eur-cents` parameter. For convenience and real-time accuracy, it would be beneficial to fetch the current BTC price directly from a reliable API.

**Describe the solution you'd like**
Implement a feature in `sim.rs` to fetch the current BTC price from the CoinGecko API when a specific flag (or `--price-eur-cents 0`) is used. This will require adding a lightweight HTTP client like `reqwest` or `attohttpc`.

**Describe alternatives you've considered**
None. CoinGecko is the standard source for crypto prices in the Beer of Satoshi ecosystem.

**Example usage**
```bash
cargo run --bin sim -- --fetch-price --cap-eur-cents 1000000
```

**Additional context**
The README already notes this as an upcoming feature:
- [ ] **SIM**: Fetch BTC price from CoinGecko
