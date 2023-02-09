pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len > digits.len() {
        return vec![];
    }

    let mut output: Vec<String> = Vec::new();

    if len == 0 {
        for _ in 0..=digits.len() {
            output.push("".to_string());
        }

        return output;
    }

    let bound = digits.len() - len;

    for i in 0..=bound {
        output.push(digits[i..i + len].to_string());
    }

    output
}
