use std::collections::HashMap;

const NUCLEOTIDES: [char; 4] = ['A', 'C', 'G', 'T'];

fn is_valid_nucleotide(nucleotide: char) -> bool {
    NUCLEOTIDES.iter().any(|x| *x == nucleotide)
}

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    // verify that the request nucleotide is valid
    if !is_valid_nucleotide(nucleotide) {
        return Err(nucleotide);
    }

    // actually count instances of the given nucleotide,
    // assuming the given dna string is valid
    match dna.chars().find(|x| !is_valid_nucleotide(*x)) {
        Some(x) => return Err(x),
        None => Ok(dna.chars().filter(|x| *x == nucleotide).count()),
    }
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut hm: HashMap<char, usize> = HashMap::new();

    for n in NUCLEOTIDES {
        match count(n, dna) {
            Ok(c) => {
                hm.insert(n, c);
            }
            Err(e) => return Err(e),
        }
    }

    Ok(hm)
}
