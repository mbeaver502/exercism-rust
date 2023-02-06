pub fn build_proverb(list: &[&str]) -> String {
    if list.len() == 0 {
        return String::new();
    }

    let mut output = String::new();

    for idx in 0..list.len() - 1 {
        output.push_str(&format!(
            "For want of a {} the {} was lost.\n",
            list[idx],
            list[idx + 1]
        ));
    }

    // We can safely unwrap because we know
    // list is guaranteed to have at least one item
    output.push_str(&format!(
        "And all for the want of a {}.",
        list.first().unwrap()
    ));

    output
}
