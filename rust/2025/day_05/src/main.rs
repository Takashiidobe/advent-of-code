use std::cmp::{max, min};

#[allow(dead_code)]
const EXAMPLE: &str = include_str!("example.txt");
#[allow(dead_code)]
const INPUT: &str = include_str!("input.txt");

fn parse(input: &str) -> (Vec<(i64, i64)>, Vec<i64>) {
    let split: Vec<_> = input.split("\n\n").collect();
    let mut ranges = vec![];
    let ingredients: Vec<i64> = split[1].lines().map(|x| x.parse().unwrap()).collect();
    for line in split[0].lines() {
        let split_line: Vec<_> = line.split('-').collect();
        let start: i64 = split_line[0].parse().unwrap();
        let end: i64 = split_line[1].parse().unwrap();
        ranges.push((start, end));
    }

    ranges.sort();

    let mut merged = vec![ranges[0]];

    for (curr_start, curr_end) in &ranges[1..] {
        let last_index = merged.len() - 1;
        let (prev_start, prev_end) = merged[last_index];

        if prev_end >= *curr_start {
            merged[last_index] = (min(prev_start, *curr_start), max(*curr_end, prev_end));
        } else {
            merged.push((*curr_start, *curr_end));
        }
    }

    (merged, ingredients)
}

fn part_1(input: &str) -> i64 {
    let (ranges, ingredients) = parse(input);
    let mut count = 0;
    for ingredient in ingredients {
        let index = ranges.binary_search_by_key(&ingredient, |&(a, _)| a);

        match index {
            Ok(_) => count += 1,
            Err(i) => {
                let prev = i.saturating_sub(1);
                let (prev_start, prev_end) = ranges[prev];
                let (curr_start, curr_end) = ranges[i.min(ranges.len() - 1)];
                if ingredient >= prev_start && ingredient <= prev_end
                    || ingredient >= curr_start && ingredient <= curr_end
                {
                    count += 1;
                }
            }
        }
    }
    count
}

fn part_2(input: &str) -> i64 {
    let (ranges, _) = parse(input);
    ranges.iter().map(|(start, end)| end - start + 1).sum()
}

fn main() {
    println!("Part 1 Example: {}", part_1(EXAMPLE));
    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2 Example: {}", part_2(EXAMPLE));
    println!("Part 2: {}", part_2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {
        assert_eq!(
            parse(EXAMPLE),
            (vec![(3, 5), (10, 20)], vec![1, 5, 8, 11, 17, 32])
        );
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EXAMPLE), 3);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(EXAMPLE), 14);
    }
}
