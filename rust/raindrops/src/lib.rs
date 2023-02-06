pub fn raindrops(n: u32) -> String {
    let mut output = String::new();

    let div_3 = n % 3 == 0;
    let div_5 = n % 5 == 0;
    let div_7 = n % 7 == 0;

    if div_3 {
        output.push_str("Pling");
    }

    if div_5 {
        output.push_str("Plang");
    }

    if div_7 {
        output.push_str("Plong");
    }

    if !(div_3 || div_5 || div_7) {
        output.push_str(&n.to_string());
    }

    output
}
