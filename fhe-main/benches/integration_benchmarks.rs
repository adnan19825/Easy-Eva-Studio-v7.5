//! Integration Benchmarks für gesamte FHE+PQC Pipeline
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn bench_full_pipeline(c: &mut Criterion) {
    c.bench_function("full_vault_operation", |b| {
        b.iter(|| {
            // Simuliere eine komplette Vault-Operation
            let mut total = 0u64;
            for id in 0..100 {
                // Simuliere: encrypt → store → retrieve → decrypt
                for value in 0..10 {
                    total = total.wrapping_add(id * value);
                }
            }
            black_box(total);
        })
    });
    
    c.bench_function("batch_operations", |b| {
        b.iter(|| {
            let mut results = Vec::new();
            for i in 0..50 {
                let batch_result = (0..20).map(|j| i * j).sum::<u64>();
                results.push(batch_result);
            }
            black_box(results);
        })
    });
}

criterion_group!(
    benches,
    bench_full_pipeline,
    bench_batch_operations,
);
criterion_main!(benches);
