use filterlab_ref::BloomFilter;

#[test]
fn simple_test() {
    // https://hur.st/bloomfilter/?n=12&p=&m=128&k=1
    let mut bf = BloomFilter::new(128, 1);

    for i in (1..=12).filter(|n| n % 3 == 0) {
        bf.insert(&i);
    }

    for i in (1..=12).filter(|n| n % 3 == 0) {
        assert!(
            bf.contains(&i),
            "Bloom filters must not have false negatives"
        );
    }

    let mut false_positives = 0;
    for i in (1..12).filter(|n| n % 3 != 0) {
        if bf.contains(&i) {
            false_positives += 1;
        }
    }

    // Given the stats of the bloom filter, the false positive rate should be no more than 1 in 11.
    // If we are checking 4 elements, there shouldn't be more than 1 false positive.
    // We make it 2 elements for some wiggle room.
    assert!(
        false_positives <= 2,
        "Encountered {false_positives} false positives, should be no more than 2"
    );
}


#[test]
fn medium_test() {
    // https://hur.st/bloomfilter/?n=100&p=&m=1024&k=1
    let mut bf = BloomFilter::new(1024, 1);

    for i in (1..=300).filter(|n| n % 3 == 0) {
        bf.insert(&i);
    }

    for i in (1..=300).filter(|n| n % 3 == 0) {
        assert!(
            bf.contains(&i),
            "Bloom filters must not have false negatives"
        );
    }

    let mut false_positives = 0;
    for i in (1..300).filter(|n| n % 3 != 0) {
        if bf.contains(&i) {
            false_positives += 1;
        }
    }

    // Given the stats of the bloom filter, the false positive rate should be no more than 1 in 11.
    // If we are checking 200 elements, there shouldn't be more than 20 false positives.
    // We make it 30 elements for some wiggle room.
    assert!(
        false_positives <= 30,
        "Encountered {false_positives} false positives, should be no more than 20-30"
    );
}
