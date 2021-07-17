fn main() {
    let v1 = vec![1,2,3];

    let v2: Vec<_> = v1.iter().map(|x| x+1).collect();

    println!("{:?}", v2);
    println!("{:?}", v1);

    using_other_iterator_trait_methods();
}


struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0}
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

fn calling_next_directly() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}

fn using_other_iterator_trait_methods() {
    let v: Vec<u32> = Counter::new().zip(Counter::new().skip(1))
                                 .map(|(a, b) | a * b)
                                 .collect();
    let v_iter = v.iter();

    for x in v_iter {
        println!("{:?}", x);
    }

    let sum: u32 = Counter::new().zip(Counter::new().skip(1))
                                 .map(|(a, b) | a * b)
                                 .filter(|x| x%3 == 0)
                                 .sum();

    assert_eq!(18, sum);
}
