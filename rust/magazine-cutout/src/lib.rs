// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::{collections::HashMap, hash::Hash, ops::Add};

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut magazine_map = HashMap::<&str, u32>::new();
    let mut note_map = HashMap::<&str, u32>::new();

    note.iter().for_each(|&word| {
        note_map.entry(word).and_modify(|x| *x += 1).or_insert(1);
    });

    magazine.iter().for_each(|&word| {
        magazine_map
            .entry(word)
            .and_modify(|x| *x += 1)
            .or_insert(1);
    });

    for (word, count) in note_map {
        if !magazine_map.contains_key(word) {
            return false;
        }

        if magazine_map[word] < count {
            return false;
        }
    }


    return true;
}
