---
name: Bug report
about: The `eur_cap_cents` parameter only caps Tier F prizes, leading to budget overruns.
title: '[BUG] Financial Risk: `eur_cap_cents` does not cap total promotion cost'
labels: bug, security, financial-risk
assignees: ''

---

**Describe the bug**
The `generate_distribution` function takes a `eur_cap_cents` parameter, which the documentation implies is a total budget cap. However, the implementation only applies this cap to the **Tier F** reward pool. The rewards for Tiers A-E are fixed at **7,100,000 satoshis** regardless of the cap or BTC price.

**To Reproduce**
Steps to reproduce the behavior:
1. Set BTC price to €100,000 (`10_000_000` cents).
2. Set `eur_cap_cents` to €10,000 (`1,000,000` cents).
3. The fixed cost of Tiers A-E is 7.1M sats = €7,100.
4. The Tier F pool is then allocated up to €10,000 (if the cap is satisfied).
5. Total promotion cost: **€17,100**, which is 71% higher than the provided cap.

**Expected behavior**
The `eur_cap_cents` should represent the **total maximum expenditure** of the promotion, or the parameter should be renamed to `tier_f_cap_eur_cents` to avoid misleading the user.

**Code sample**
```rust
let price = 10_000_000; // €100k BTC
let cap = 1_000_000;    // €10k Total Cap?
let dist = generate_distribution(price, cap).unwrap();

let total_sats: u128 = dist.iter().map(|b| b.sats as u128).sum();
let total_cents = total_sats * price as u128 / 100_000_000;
println!("Total cost in cents: {}", total_cents); // Will output ~1,710,000 cents
```

**Environment:**
- bos-algo version: 0.1.0

**Additional context**
This is a critical financial risk as it could lead to the promotion sponsor spending significantly more than their intended budget.
