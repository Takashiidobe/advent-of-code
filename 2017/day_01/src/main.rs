use std::fs::read_to_string;

fn parse() -> String {
    read_to_string("../input.txt").unwrap()
}

fn part_one() -> u32 {
    let input = parse();
    let input: Vec<u32> = input
        .chars()
        .filter(|c| c.is_ascii_digit())
        .map(|d| d.to_digit(10).unwrap())
        .collect();
    let first = input[0];
    let last = input[input.len() - 1];

    let mut total = 0;

    if first == last {
        total += first;
    }

    let mut i = 0;
    let mut j = 1;

    while j < input.len() {
        if input[i] == input[j] {
            total += input[i];
        }
        i += 1;
        j += 1;
    }

    total
}

fn part_two() -> usize {
    let input = parse();
    let v: Vec<usize> = input
        .chars()
        .filter(|c| c.is_ascii_digit())
        .map(|d| d.to_digit(10).unwrap() as usize)
        .collect();

    let mid_index = input.len() / 2;
    let mut total = 0;
    let mut first_half = vec![];
    let mut second_half = vec![];

    for i in 0..mid_index {
        first_half.push(v[i]);
    }

    for i in mid_index..v.len() {
        second_half.push(v[i]);
    }

    for i in 0..mid_index {
        if first_half[i] == second_half[i] {
            total += first_half[i] + second_half[i];
        }
    }

    total
}

fn main() {
    println!("part one: {}", part_one());
    println!("Part two: {}", part_two());
}
