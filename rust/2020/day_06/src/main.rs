#![feature(iterator_fold_self)]
use std::collections::HashSet;
use std::fs::read_to_string;

fn parse() -> String {
    read_to_string("../input.txt").unwrap()
}

fn part_one() -> usize {
    let input = parse();
    let mut total = 0;

    let split_input = input.split("\n\n").collect::<Vec<&str>>();

    for string in split_input {
        let mut set = HashSet::new();
        for c in string.chars() {
            if c != '\n' {
                set.insert(c);
            }
        }
        total += set.len();
    }
    total
}

fn part_two() -> usize {
    let input = parse();
    let mut total = 0;

    let mut answers = vec![];
    let mut group = vec![];

    for line in input.lines() {
        if line.is_empty() {
            answers.push(group);
            group = vec![];
        } else {
            let mut chars = HashSet::new();
            for c in line.chars() {
                chars.insert(c);
            }
            group.push(chars);
        }
    }

    answers.push(group);

    for group in answers {
        let count = group
            .clone()
            .into_iter()
            .fold_first(|a, b| a.intersection(&b).map(|c| c.to_owned()).collect())
            .unwrap()
            .len();
        total += count;
    }

    total
}

fn main() {
    println!("part one: {}", part_one());
    println!("part two: {}", part_two());
}
