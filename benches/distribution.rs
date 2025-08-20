use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use bos_algo::generate_distribution;

fn benchmark_distribution(c: &mut Criterion) {
    let mut group = c.benchmark_group("distribution_generation");
    
    // Test different BTC prices
    let btc_prices = vec![
        10_000_00,    // €10,000
        50_000_00,    // €50,000
        96_496_00,    // €96,496 (example from docs)
        200_000_00,   // €200,000
    ];
    
    // Benchmark with no cap
    for price in &btc_prices {
        group.bench_with_input(
            BenchmarkId::new("no_cap", format!("€{}", price / 100)),
            price,
            |b, &price| {
                b.iter(|| {
                    generate_distribution(black_box(price), black_box(0))
                });
            },
        );
    }
    
    // Benchmark with various caps
    let caps = vec![
        1_000_00,     // €1,000
        10_000_00,    // €10,000
        100_000_00,   // €100,000
    ];
    
    let base_price = 96_496_00; // €96,496
    for cap in &caps {
        group.bench_with_input(
            BenchmarkId::new("with_cap", format!("€{}", cap / 100)),
            cap,
            |b, &cap| {
                b.iter(|| {
                    generate_distribution(black_box(base_price), black_box(cap))
                });
            },
        );
    }
    
    group.finish();
}

fn benchmark_edge_cases(c: &mut Criterion) {
    let mut group = c.benchmark_group("edge_cases");
    
    // Minimum viable cap (just enough for 21 sats per F bottle)
    let price = 96_496_00;
    let min_cap = (28_389u128 * 21 * price as u128 / 100_000_000) as u64;
    
    group.bench_function("minimum_viable_cap", |b| {
        b.iter(|| {
            generate_distribution(black_box(price), black_box(min_cap))
        });
    });
    
    // Very high BTC price
    group.bench_function("high_btc_price", |b| {
        b.iter(|| {
            generate_distribution(black_box(1_000_000_00), black_box(0))
        });
    });
    
    // Very low BTC price
    group.bench_function("low_btc_price", |b| {
        b.iter(|| {
            generate_distribution(black_box(1_00), black_box(0))
        });
    });
    
    group.finish();
}

fn benchmark_memory(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_usage");
    
    // Measure allocation patterns
    group.bench_function("allocation_pattern", |b| {
        b.iter(|| {
            let dist = generate_distribution(black_box(96_496_00), black_box(10_000_00)).unwrap();
            // Force the compiler to not optimize away the distribution
            assert_eq!(dist.len(), 31_500);
            dist
        });
    });
    
    group.finish();
}

criterion_group!(
    benches,
    benchmark_distribution,
    benchmark_edge_cases,
    benchmark_memory
);
criterion_main!(benches);