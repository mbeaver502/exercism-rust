pub fn factors(n: u64) -> Vec<u64> {
    if n == 1 {
        return vec![];
    } else if n == 2 {
        return vec![2];
    }

    let mut output: Vec<u64> = Vec::new();

    let bound = ((n as f64).sqrt().floor()) + 1.0;
    let bound: u64 = bound as u64;

    let mut m = n;

    // dubious implementation...
    for i in 2..=bound {
        while m % i == 0 {
            output.push(i);
            m /= i;
        }
    }

    if n % m == 0 && m != 1 {
        output.push(m);
    }

    output
}
