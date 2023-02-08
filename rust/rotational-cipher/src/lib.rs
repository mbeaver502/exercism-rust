fn get_char(alphabet: &str, c: char, key: i8) -> char {
    let alphabet_len: i8 = alphabet.len() as i8;

    match alphabet.find(c) {
        Some(p) => {
            // guarantee we'll get something within our alphabet's bounds
            let mut idx: i8 = p as i8 + key;
            idx = idx.rem_euclid(alphabet_len);

            alphabet.chars().nth(idx as usize).unwrap()
        }
        None => c,
    }
}

pub fn rotate(input: &str, key: i8) -> String {
    const ALPHABET_LOWER: &str = "abcdefghijklmnopqrstuvwxyz";
    const ALPHABET_UPPER: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

    let mut output = String::new();

    for c in input.chars() {
        if c.is_uppercase() {
            output.push(get_char(ALPHABET_UPPER, c, key));
        } else if c.is_lowercase() {
            output.push(get_char(ALPHABET_LOWER, c, key));
        } else {
            output.push(c);
        }
    }

    output
}
