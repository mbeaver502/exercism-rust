pub fn is_armstrong_number(num: u32) -> bool {
    if num < 10 {
        return true;
    }

    let num_digits = num.ilog(10) + 1;

    let mut n: u64 = num as u64;
    let mut sum: u64 = 0;

    while n > 0 {
        sum += (n % 10).pow(num_digits);
        n /= 10;
    }

    sum == num as u64
}
