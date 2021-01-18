use std::fs::read_to_string;
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

fn keys_match<T: Eq + Hash, U, V>(map1: &HashMap<T, U>, map2: &HashMap<T, V>) -> bool {
    map1.len() == map2.len() && map1.keys().all(|k| map2.contains_key(k))
}

fn parse() -> String {
    read_to_string("../input.txt").unwrap()
}

fn part_one() -> u32 {
    let input = parse();
    let mut count = 0;

    for input in input.lines() {
        let passphrases: Vec<&str> = input.clone().split(" ").collect();
        let mut set = HashSet::new();

        for phrase in passphrases.clone() {
            set.insert(phrase);
        }

        if set.len() == passphrases.len() {
            count += 1;
        }
    }
    count
}

fn part_two() -> u32 {
    let input = parse();
    let mut count = 0;
    for input in input.lines() {
        let passphrases: Vec<&str> = input.clone().split(" ").collect();

        let mut sorted = vec![];
        for phrase in passphrases.clone() {
            let mut split: Vec<&str> = phrase.split("").filter(|c| *c != "").collect();
            split.sort();
            sorted.push(split.join(""));
        }

        let mut hash_set = HashSet::new();

        for item in sorted.clone() {
            hash_set.insert(item);
        }

        if hash_set.len() == sorted.len() {
            count += 1;
        }
    }

    count
}

fn main() {
    println!("part one: {}", part_one());
    println!("part two: {}", part_two());
}
