use criterion::{black_box, criterion_group, criterion_main, Criterion};
use filterlab_ref::BloomFilter;
use rand::distributions::{Bernoulli, Distribution, Standard};
use rand::Rng;
use std::time::Duration;

/// Hack: this should be enough operations so that the bencher doesn't run out of work.
const MEGABYTE: usize = 1 << 20;

pub fn bloom_filter_read_benchmark(c: &mut Criterion) {
    let coin = Bernoulli::new(0.5).expect("0.5 is in between 0 and 1");
    let zipf = zipf::ZipfDistribution::new(MEGABYTE - 1, 1.1).unwrap();

    let mut rng = rand::thread_rng();

    // Generate 1 million random integers.
    let list: Vec<i32> = rand::thread_rng()
        .sample_iter(Standard)
        .take(MEGABYTE)
        .collect();

    // Create the bloom filter up front. We don't want to measure write speed for this benchmark.
    let mut bf = BloomFilter::new(MEGABYTE * 8, 6);
    for elem in &list {
        bf.insert(elem);
    }

    // We don't want to be measuring the speed of RNG, so figure out all the lookups beforehand.
    let lookups: Vec<i32> = (0..MEGABYTE)
        .map(|_| {
            if coin.sample(&mut rng) {
                // Choose a random element in the list.
                let index = zipf.sample(&mut rng);
                list[index]
            } else {
                // Lookup a random element.
                rng.gen()
            }
        })
        .collect();

    let mut index = 0;
    c.bench_function("read", |b| {
        b.iter(|| {
            let elem = lookups[index];
            index += 1;

            if bf.contains(black_box(&elem)) {
                // The next line could be very expensive if there was a false positive.
                let found_index = list.iter().position(|&x| x == elem);

                // Make sure the compiler doesn't optimize this out.
                black_box(found_index);
            }
        })
    });
}

criterion_main!(benches);
criterion_group! {
    name = benches;
    config = Criterion::default().warm_up_time(Duration::new(0, 100000)).measurement_time(Duration::new(5, 0));
    targets = bloom_filter_read_benchmark
}
