use criterion::{black_box, criterion_group, criterion_main, Criterion};
use filterlab::BloomFilter;
use rand::distributions::Standard;
use rand::Rng;

/// Approximately equal to 1 million.
const MEGABYTE: usize = 1 << 20;
/// Approximately equal to 1 billion.
const GIGABYTE: usize = 1 << 30;

/// This benchmark tests the performance of `BloomFilter::insert`.
pub fn bloom_filter_write_benchmark(c: &mut Criterion) {
    // Generate 1 million random integers.
    let list: Vec<i32> = rand::thread_rng()
        .sample_iter(Standard)
        .take(MEGABYTE)
        .collect();

    // Allocate the bloom filter.
    let mut bf = BloomFilter::new(GIGABYTE, 64);

    let mut index = 0;
    c.bench_function("write", |b| {
        b.iter(|| {
            bf.insert(black_box(&list[index % list.len()]));
            index += 1; // If only we had a way to infinitely cycle through this iterator...
        })
    });

    black_box(bf);
}

criterion_main!(benches);
criterion_group! {
    name = benches;
    config = Criterion::default();
    targets = bloom_filter_write_benchmark
}
