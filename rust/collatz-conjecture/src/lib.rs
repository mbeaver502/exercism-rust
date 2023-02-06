pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }

    if n == 1 {
        return Some(0);
    }

    let mut tmp = n;
    let mut count: u64 = 0;

    while tmp != 1 {
        match tmp % 2 {
            0 => tmp /= 2,
            _ => match tmp.checked_mul(3) {
                Some(v) => match v.checked_add(1) {
                    Some(x) => tmp = x,
                    None => return None,
                },
                None => return None,
            },
        };

        count += 1;
    }

    Some(count)
}
