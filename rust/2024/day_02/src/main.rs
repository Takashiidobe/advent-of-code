const INPUT: &str = include_str!("input.txt");

fn parse(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect()
}

fn valid_line(level: &[i64]) -> bool {
    let is_forward_sorted = level.is_sorted_by(|a, b| a < b);
    let is_valid_forward = level.windows(2).all(|w| w[1] - w[0] <= 3);
    let is_backward_sorted = level.is_sorted_by(|a, b| a > b);
    let is_valid_backward = level.windows(2).all(|w| w[0] - w[1] <= 3);

    is_forward_sorted && is_valid_forward || is_backward_sorted && is_valid_backward
}

fn part_1(input: &str) -> i64 {
    let parsed = parse(input);
    parsed
        .iter()
        .fold(0, |acc, level| acc + if valid_line(level) { 1 } else { 0 })
}

fn part_2(input: &str) -> i64 {
    let parsed = parse(input);
    let mut res = 0;

    for mut level in parsed {
        if valid_line(&level) {
            res += 1;
            continue;
        }
        for i in 0..level.len() {
            let removed = level.remove(i);
            if valid_line(&level) {
                res += 1;
                level.insert(i, removed);
                break;
            }
            level.insert(i, removed);
        }
    }

    res
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
            vec![
                vec![7, 6, 4, 2, 1],
                vec![1, 2, 7, 8, 9],
                vec![9, 7, 6, 2, 1],
                vec![1, 3, 2, 4, 5],
                vec![8, 6, 4, 4, 1],
                vec![1, 3, 6, 7, 9]
            ]
        );
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EXAMPLE_INPUT), 2);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(EXAMPLE_INPUT), 4);
    }
}
