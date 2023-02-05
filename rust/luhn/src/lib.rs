/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let code = code.trim();

    if code.len() <= 1 {
        return false;
    }

    if code.chars().filter(|x| x.is_ascii_digit()).count()
        != code.chars().filter(|x| !x.is_whitespace()).count()
    {
        return false;
    }

    // so unnecessarily complicated, it's hard to resist
    code.chars()
        .filter(|c| !c.is_whitespace())
        .rev()
        .enumerate()
        .map(|(i, c)| {
            let mut d = c.to_digit(10).unwrap();

            if i % 2 == 1 {
                d *= 2;

                if d >= 10 {
                    d -= 9;
                }
            }

            d
        })
        .sum::<u32>()
        % 10
        == 0
}
