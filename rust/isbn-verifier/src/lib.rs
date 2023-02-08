/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    const EXPECTED_LEN: usize = 10;

    // lazy parsing of input, probably memory inefficient
    let isbn = isbn
        .replace("-", "")
        .replace(" ", "")
        .to_uppercase()
        .trim()
        .to_string();

    // ensure the input is the correct length
    if isbn.len() != EXPECTED_LEN {
        return false;
    }

    // ensure valid chars only
    if isbn.chars().any(|c| "01234567890X".contains(c) == false) {
        return false;
    }

    // ensure if X appears, it's only as check digit
    match isbn.chars().position(|x| x == 'X') {
        Some(p) => {
            if p != EXPECTED_LEN - 1 {
                return false;
            }
        }
        None => {}
    }

    // verify the checksum
    isbn.chars()
        .enumerate()
        .map(|(i, c)| {
            let n = match c {
                'X' => 10,
                _ => c.to_digit(10).unwrap(),
            };

            n * (10 - i as u32)
        })
        .sum::<u32>()
        % 11
        == 0
}
