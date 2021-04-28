use std::{collections::HashSet, fs::read_to_string};

fn parse() -> Vec<u128> {
    let i = read_to_string("../input.txt").unwrap();
    i.lines().map(|i| i.parse::<u128>().unwrap()).collect()
}

fn part_one() -> u128 {
    let input = parse();

    let mut i = 0;
    let mut j = 25;

    let mut s = HashSet::new();

    for item in i..j {
        s.insert(input[item]);
    }

    while j < input.len() {
        s.insert(input[j - 1]);
        let num_to_total = input[j];
        let mut cont = false;

        for item in s.clone() {
            if num_to_total >= item {
                if s.contains(&(num_to_total - item)) {
                    cont = true;
                }
            }
        }

        if !cont {
            return input[j];
        }

        s.remove(&input[i]);
        i += 1;
        j += 1;
    }

    0
}

fn part_two() -> u128 {
    let target = 32321523;
    let input = parse();

    let mut i = 0;
    let mut j = 1;

    let mut total = input[i];
    while i < j && j < input.len() {
        if total == target {
            let items = input.iter().skip(i).take(j - i);
            let min = items.clone().min().unwrap();
            let max = items.clone().max().unwrap();
            return min + max;
        }
        if total < target {
            total += input[j];
            j += 1;
        }
        if total > target {
            total -= input[i];
            i += 1;
        }
    }
    0
}

fn main() {
    println!("part one: {}", part_one());
    println!("part two: {}", part_two());
}
