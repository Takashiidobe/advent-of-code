use std::collections::HashMap;

const INPUT: &str = include_str!("input.txt");

fn parse(input: &str) -> (Vec<u64>, Vec<u64>) {
    let mut left = vec![];
    let mut right = vec![];
    for line in input.lines() {
        let nums: Vec<u64> = line.split("   ").map(|x| x.parse().unwrap()).collect();
        left.push(nums[0]);
        right.push(nums[1]);
    }
    left.sort();
    right.sort();
    (left, right)
}

fn part_1(input: &str) -> u64 {
    let (left, right) = parse(input);
    left.iter()
        .zip(right)
        .fold(0, |acc, (l, r)| acc + l.abs_diff(r))
}

fn part_2(input: &str) -> u64 {
    let (left, right) = parse(input);
    let mut counter: HashMap<u64, u64> = HashMap::new();
    for item in right {
        *counter.entry(item).or_default() += 1;
    }

    left.iter()
        .fold(0, |acc, l| acc + l * *counter.entry(*l).or_default())
}

fn main() {
    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_INPUT: &str = include_str!("example.txt");

    #[test]
    fn parse_test() {
        assert_eq!(
            parse(EXAMPLE_INPUT),
            (vec![1, 2, 3, 3, 3, 4], vec![3, 3, 3, 4, 5, 9])
        );
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EXAMPLE_INPUT), 11);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(EXAMPLE_INPUT), 31);
    }
}
