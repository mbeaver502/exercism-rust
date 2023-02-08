#[derive(Debug, PartialEq, Eq)]
pub struct Dna {
    // ideally wouldn't use String,
    // but too lazy to figure out
    // how to make the lifetimes
    // work for &str
    dna: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rna {
    rna: String,
}

impl Dna {
    pub fn new(dna: &str) -> Result<Self, usize> {
        match dna.chars().position(|c| "ATCG".contains(c) == false) {
            Some(p) => return Err(p),
            None => {}
        }

        Ok(Self {
            dna: String::from(dna),
        })
    }

    pub fn into_rna(self) -> Rna {
        let rna: String = self
            .dna
            .chars()
            .map(|c| match c {
                'A' => 'U',
                'T' => 'A',
                'C' => 'G',
                'G' => 'C',
                _ => c,
            })
            .into_iter()
            .collect();

        // theortically, the RNA should be OK if the DNA is OK
        Rna::new(&rna).ok().unwrap()
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Self, usize> {
        match rna.chars().position(|c| "UAGC".contains(c) == false) {
            Some(p) => return Err(p),
            None => {}
        }

        Ok(Self {
            rna: String::from(rna),
        })
    }
}
