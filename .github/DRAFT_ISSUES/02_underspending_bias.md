---
name: Feature request
about: Improve budget utilization for Tier F rewards.
title: '[FEATURE] Implement Target-Seeking Distribution for Tier F'
labels: enhancement, statistics
assignees: ''

---

**Is your feature request related to a problem? Please describe.**
The current Tier F allocation algorithm uses a uniform random distribution (`21..=feasible_max`) at each step. While this ensures the budget cap is never exceeded, it results in a strong statistical bias toward the **mid-point (~260 sats)** of the allowed range. If the budget cap allows for a higher average (e.g., 400 sats per bottle), the algorithm will consistently "underspend" the budget, potentially leaving a significant portion of the intended prize pool unallocated.

**Describe the solution you'd like**
Implement a "target-seeking" mode that aims to hit the `eur_cap_cents` budget as closely as possible without exceeding it. This would involve adjusting the random selection to favor higher values if the remaining budget is disproportionately high, while still maintaining some degree of entropy and fairness.

**Describe alternatives you've considered**
An alternative would be to simply warn the user that the budget cap is a **ceiling, not a target**, but this might be counter-intuitive for marketing teams who have a specific fixed amount they want to give away.

**Example usage**
```rust
// New parameter or mode to indicate target-seeking
let dist = generate_distribution_full_budget(price, cap, Mode::TargetSeek).unwrap();
```

**Additional context**
For 28,389 Tier F bottles, the current algorithm will spend approximately 7.4M sats regardless of the cap size (if the cap is high). If the user provided a cap of 12M sats, the remaining 4.6M sats will stay unspent.
