fn is_prime(n: u32) -> bool {
    if n > 0 && n <= 2 {
        return true;
    }

    let bound = (n as f32).sqrt().floor();
    let bound = bound as u32;

    for i in 2..=bound {
        if n % i == 0 {
            return false;
        }
    }

    true
}

pub fn nth(n: u32) -> u32 {
    let mut nth_prime: u32 = 2;
    let mut count: u32 = 0;
    let mut i: u32 = 3;

    while count < n {
        if is_prime(i) {
            nth_prime = i;
            count += 1;
        }
        i += 1;
    }

    nth_prime
}
