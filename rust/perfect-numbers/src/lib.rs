use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None;
    }

    let mut factors = HashSet::new();

    let bound = (num as f64).sqrt().floor() + 1.0;
    let bound: u64 = bound as u64;

    for i in 1..=bound {
        if num % i == 0 {
            factors.insert(i);
            factors.insert(num / i);
        }
    }

    let aliquot: u64 = factors.iter().filter(|&x| *x != num).sum();

    if aliquot == num {
        Some(Classification::Perfect)
    } else if aliquot > num {
        Some(Classification::Abundant)
    } else {
        Some(Classification::Deficient)
    }
}
