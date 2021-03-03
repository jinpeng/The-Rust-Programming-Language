pub struct Counter {
    count: u32,
}

impl Counter {
    pub fn new() -> Counter {
        Counter {
            count: 0
        }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            return Some(self.count)
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::counter::Counter;

    #[test]
    fn calling_next_directly() {
        let mut counter = Counter::new();
        assert_eq!(Some(1), counter.next());
        assert_eq!(Some(2), counter.next());
        assert_eq!(Some(3), counter.next());
        assert_eq!(Some(4), counter.next());
        assert_eq!(Some(5), counter.next());
        assert_eq!(None, counter.next());
    }

    #[test]
    fn using_other_iterator_trait_methods() {
        let sum: u32 = Counter::new()
            .zip(Counter::new().skip(1))
            .map(|(a, b)| a * b)
            .filter(|x| x % 3 == 0)
            .sum();
        assert_eq!(18, sum);
    }
}
