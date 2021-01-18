use std::{collections::HashMap, fs::read_to_string};

fn parse() -> Vec<usize> {
    let mut input: String = read_to_string("../input.txt").unwrap();
    input.pop().unwrap();

    input
        .split('\t')
        .map(|c| c.parse::<usize>().unwrap())
        .collect()
}

fn part_one() -> u32 {
    let mut input = parse();

    let mut set: HashMap<Vec<usize>, u32> = HashMap::new();
    let mut count = 0;

    while !set.contains_key(&input.clone()) {
        set.insert(input.clone(), 1);
        let mut index: usize = 0;
        let mut max_so_far: usize = 0;
        for (i, item) in input.clone().into_iter().enumerate() {
            if item > max_so_far {
                index = i;
                max_so_far = item;
            }
        }
        input[index] = 0;

        for _ in 0..max_so_far {
            index = (index + 1) % input.len();
            input[index] += 1;
        }

        count += 1;
    }
    count
}

fn part_two() -> u32 {
    let mut input = parse();
    let mut count = 0;
    let mut map: HashMap<Vec<usize>, u32> = HashMap::new();

    while *map.entry(input.clone()).or_insert(1) <= 2 {
        *map.get_mut(&input).unwrap() += 1;
        let mut index: usize = 0;
        let mut max_so_far: usize = 0;
        for (i, item) in input.clone().into_iter().enumerate() {
            if item > max_so_far {
                index = i;
                max_so_far = item;
            }
        }
        input[index] = 0;

        for _ in 0..max_so_far {
            index = (index + 1) % input.len();
            input[index] += 1;
        }

        count += 1;
    }

    count - part_one()
}

fn main() {
    println!("part one: {}", part_one());
    println!("part two: {}", part_two());
}
