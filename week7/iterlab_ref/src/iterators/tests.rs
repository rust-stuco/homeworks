mod cycle_tests {
    use super::super::cycle::*;

    #[test]
    fn basic_cycling() {
        {
            let numbers = [1, 2, 3];
            let mut cycle = Cycle::new(numbers.iter().cloned());

            assert_eq!(cycle.next(), Some(1));
            assert_eq!(cycle.next(), Some(2));
            assert_eq!(cycle.next(), Some(3));
            assert_eq!(cycle.next(), Some(1)); // Cycle back to the beginning
            assert_eq!(cycle.next(), Some(2));
            assert_eq!(cycle.next(), Some(3));

            for i in 0..1000 {
                assert_eq!(cycle.next(), Some((i % 3) + 1))
            }
        }

        {
            let mut cycle = Cycle::new(std::iter::repeat(1));

            // Only run a limited number of iterations to avoid an infinite loop
            for _ in 0..10 {
                assert_eq!(cycle.next(), Some(1));
            }
        }
    }

    #[test]
    fn empty_cycle() {
        {
            let binding = Vec::<()>::new();
            let empty = binding.iter().cloned();
            let mut cycle = Cycle::new(empty);
            assert_eq!(cycle.next(), None); // Should always yield None
        }

        {
            let mut cycle = Cycle::new(std::iter::empty::<i32>());
            assert_eq!(cycle.next(), None);
        }

        {
            let numbers = [1, 2, 3];
            let mut original_iter = numbers.iter().cloned();
            original_iter.next();
            original_iter.next();
            original_iter.next(); // Consume all elements

            let mut cycle = Cycle::new(original_iter);

            assert_eq!(cycle.next(), None); // Should still be None after the cycle
        }
    }

    #[test]
    fn partially_consumed_cycle() {
        let numbers = [1, 2, 3];
        let mut original_iter = numbers.iter().cloned();
        assert_eq!(original_iter.next(), Some(1)); // Consume one item

        let mut cycle = Cycle::new(original_iter);

        assert_eq!(cycle.next(), Some(2)); // Should start from the remaining items
        assert_eq!(cycle.next(), Some(3));
        assert_eq!(cycle.next(), Some(2)); // Cycle back to remaining items
        assert_eq!(cycle.next(), Some(3));
        assert_eq!(cycle.next(), Some(2));
        assert_eq!(cycle.next(), Some(3));
    }

    #[test]
    fn cycle_composed() {
        {
            let numbers = [1, 2, 3, 4, 5];
            let mut cycle = Cycle::new(numbers.iter().cloned().filter(|x| x % 2 == 0));

            assert_eq!(cycle.next(), Some(2));
            assert_eq!(cycle.next(), Some(4));
            assert_eq!(cycle.next(), Some(2)); // Cycle back to the filtered elements
            assert_eq!(cycle.next(), Some(4));
            assert_eq!(cycle.next(), Some(2));
        }

        {
            let numbers = [1, 2, 3, 4, 5];
            let mut cycle = Cycle::new(numbers.chunks(2));

            assert_eq!(cycle.next(), Some(&[1, 2][..]));
            assert_eq!(cycle.next(), Some(&[3, 4][..]));
            assert_eq!(cycle.next(), Some(&[5][..])); // Last chunk might be smaller
            assert_eq!(cycle.next(), Some(&[1, 2][..])); // Cycle back to the beginning
        }
    }
    #[test]
    fn mutating_inner_iterator() {
        #[derive(Clone)]
        struct DoublingIterator {
            values: Vec<i32>,
            index: usize,
        }

        impl Iterator for DoublingIterator {
            type Item = i32;

            fn next(&mut self) -> Option<Self::Item> {
                if self.index < self.values.len() {
                    self.values[self.index] *= 2; // Mutate the value
                    self.index += 1;
                    Some(self.values[self.index - 1])
                } else {
                    None
                }
            }
        }

        let iterator = DoublingIterator {
            values: vec![1, 2, 3],
            index: 0,
        };

        let mut cycle = Cycle::new(iterator);

        assert_eq!(cycle.next(), Some(2));
        assert_eq!(cycle.next(), Some(4));
        assert_eq!(cycle.next(), Some(6));
        assert_eq!(cycle.next(), Some(2));
        assert_eq!(cycle.next(), Some(4));
        assert_eq!(cycle.next(), Some(6));
    }
}
