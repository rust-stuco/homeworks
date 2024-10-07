use std::sync::atomic::{AtomicUsize, Ordering};

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use filterlab_ref::BloomFilter;
use rand::distributions::{Bernoulli, Distribution, Standard};
use rand::Rng;

const KILOBYTE: usize = 1 << 10;
// const MEGABYTE: usize = 1 << 20;

fn read_bench(elems: &[i32]) {
    const READERS: usize = 16;

    // Create the bloom filter up front. We want to partially measure write speed as well.
    let mut bf = BloomFilter::new(KILOBYTE * 8, 6);

    for elem in elems {
        bf.insert(elem);
    }

    let false_positives = AtomicUsize::new(0);
    let coin = Bernoulli::new(0.001).expect("0.001 is in between 0 and 1");
    let zipf = zipf::ZipfDistribution::new(KILOBYTE - 1, 1.1).unwrap();

    // Spawn 16 threads that read from the bloom filter to check if a value is located in the list
    // of elements.
    std::thread::scope(|s| {
        for _ in 0..READERS {
            s.spawn(|| {
                // Alternate between looking up an element guaranteed to be in the list of elements
                // and a random element.

                let mut rng = rand::thread_rng();

                for _ in 0..(KILOBYTE / READERS) {
                    let elem = if coin.sample(&mut rng) {
                        // Choose a random element in the list.
                        let index = zipf.sample(&mut rng);
                        elems[index]
                    } else {
                        // Lookup a random element.
                        rng.gen()
                    };

                    if bf.contains(&elem) {
                        // The next line could be very expensive if there was a false positive.
                        let found_index = elems.iter().position(|&x| x == elem);

                        if found_index.is_none() {
                            false_positives.fetch_add(1, Ordering::Relaxed);
                        }

                        // Make sure the compiler doesn't optimize this out.
                        black_box(found_index);
                    }
                }
            });
        }
    });
}

pub fn criterion_benchmark(c: &mut Criterion) {
    // Generate 1 million random integers.
    let values: Vec<i32> = rand::thread_rng()
        .sample_iter(Standard)
        .take(KILOBYTE)
        .collect();

    c.bench_function("read", |b| b.iter(|| read_bench(&values)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
