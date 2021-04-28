use std::fs::read_to_string;

fn parse() -> Vec<i32> {
    read_to_string("../input.txt")
        .unwrap()
        .lines()
        .map(|l| l.parse::<i32>().unwrap())
        .collect()
}

fn part_one() -> usize {
    let mut jumps = parse();
    let mut pos: i32 = 0;
    let mut count = 0;

    while pos >= 0 && (pos as usize) < jumps.len() {
        let prev_pos = pos;
        pos += jumps[pos as usize];
        jumps[prev_pos as usize] += 1;
        count += 1;
    }

    count
}

fn part_two() -> usize {
    let mut jumps = parse();
    let mut pos: i32 = 0;
    let mut count = 0;

    while pos >= 0 && (pos as usize) < jumps.len() {
        let prev_pos = pos;
        pos += jumps[pos as usize];
        if jumps[prev_pos as usize] > 2 {
            jumps[prev_pos as usize] -= 1;
        } else {
            jumps[prev_pos as usize] += 1;
        }
        count += 1;
    }

    count
}

fn main() {
    println!("part one is: {}", part_one());
    println!("part two is: {}", part_two());
}
