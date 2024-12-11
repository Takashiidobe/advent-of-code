use std::collections::HashMap;

const INPUT: &str = include_str!("input.txt");
const EXAMPLE: &str = include_str!("example.txt");

fn parse(input: &str) -> Vec<u128> {
    input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn next_round(stone: u128) -> Vec<u128> {
    if stone == 0 {
        return vec![1];
    }

    let string_stone = stone.to_string();

    if string_stone.len() % 2 == 0 {
        let mid = stone.to_string().len() / 2;
        let (left, right) = string_stone.split_at(mid);
        return vec![left.parse().unwrap(), right.parse().unwrap()];
    }

    vec![2024 * stone]
}

fn blink(stones: Vec<u128>, times: usize) -> u128 {
    let mut counter: HashMap<u128, u128> = HashMap::new();
    for stone in stones {
        *counter.entry(stone).or_default() += 1;
    }

    for _ in 0..times {
        let mut new_counter = HashMap::new();

        for (num, times) in counter {
            let next_next = next_round(num);
            for item in next_next {
                *new_counter.entry(item).or_default() += times;
            }
        }

        counter = new_counter;
    }

    counter.values().sum()
}

fn part_1(input: &str) -> u128 {
    let parsed = parse(input);
    blink(parsed, 25)
}

fn part_2(input: &str) -> u128 {
    let parsed = parse(input);
    blink(parsed, 75)
}

fn main() {
    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {
        assert_eq!(parse(EXAMPLE), vec![125, 17]);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EXAMPLE), 55312);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(EXAMPLE), 65601038650482);
    }
}
