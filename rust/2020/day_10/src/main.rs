use lazy_static::lazy_static;
use std::fs::read_to_string;

fn parse() -> Vec<u32> {
    let mut input: Vec<u32> = read_to_string("../input.txt")
        .unwrap()
        .lines()
        .map(|c| c.parse::<u32>().unwrap())
        .collect();
    input.push(0);
    input.sort();
    input.push(input.iter().last().unwrap() + 3);
    input
}

lazy_static! {
    static ref INPUT: Vec<u32> = parse();
}

fn part_one() -> u32 {
    let mut one_jolts = 0;
    let mut three_jolts = 0;
    let mut curr = 0;

    for num in INPUT.iter() {
        match num - curr {
            1 => one_jolts += 1,
            3 => three_jolts += 1,
            _ => {}
        }
        curr = *num;
    }
    three_jolts * one_jolts
}

fn part_two() -> u64 {
    let len = INPUT.len();

    let mut paths: Vec<u64> = vec![1; len];
    if INPUT[len - 1] - INPUT[len - 3] <= 3 {
        paths[len - 3] = 2;
    }

    for i in (0..len - 3).rev() {
        let mut path_sum = paths[i + 1];
        if INPUT[i + 2] - INPUT[i] <= 3 {
            path_sum += paths[i + 2]
        }
        if INPUT[i + 3] - INPUT[i] <= 3 {
            path_sum += paths[i + 3]
        }
        paths[i] = path_sum;
    }
    paths[0]
}

fn main() {
    println!("Part one: {}", part_one());
    println!("Part two: {}", part_two());
}
