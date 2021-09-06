use std::fs::read_to_string;

fn parse() -> String {
    read_to_string("../input.txt").unwrap()
}

fn part_one() -> u32 {
    let lines = parse();

    let mut total = 0;
    for line in lines.lines() {
        let split_line: Vec<u32> = line
            .split("\t")
            .map(|c| c.parse::<u32>().unwrap())
            .collect();

        let minimum = split_line.iter().min().unwrap();
        let maximum = split_line.iter().max().unwrap();

        total += maximum - minimum;
    }
    total
}

fn part_two() -> u32 {
    let lines = parse();
    let mut total = 0;
    for line in lines.lines() {
        let mut split_line: Vec<u32> = line
            .split("\t")
            .map(|c| c.parse::<u32>().unwrap())
            .collect();

        split_line.sort();

        for i in 0..split_line.len() {
            for j in 1..split_line.len() {
                let right = split_line[j];
                let left = split_line[i];
                if left != right && right % left == 0 {
                    total += right / left;
                }
            }
        }
    }

    total
}

fn main() {
    println!("part_one: {}", part_one());
    println!("part_two: {}", part_two());
}
