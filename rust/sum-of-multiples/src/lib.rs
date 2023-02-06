pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (1..limit)
        .map(|x| {
            for f in factors {
                if *f == 0 {
                    continue;
                }

                if x % *f == 0 {
                    return x;
                }
            }

            0
        })
        .sum::<u32>()
}
