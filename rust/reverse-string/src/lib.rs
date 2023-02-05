// https://crates.io/crates/unicode-reverse
use unicode_reverse::reverse_grapheme_clusters_in_place;

pub fn reverse(input: &str) -> String {
    let mut s = input.clone().to_string();

    reverse_grapheme_clusters_in_place(&mut s);

    s

    // does not handle graphemes
    //input.chars().rev().collect()
}
