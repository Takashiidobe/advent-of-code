use cached::proc_macro::cached;
use cached::SizedCache;

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

fn parse(input: &str) -> (Vec<String>, Vec<String>) {
    let split: Vec<_> = input.split("\n\n").collect();
    let (patterns, towels) = (split[0], split[1]);

    let mut patterns: Vec<String> = patterns.split(", ").map(|x| x.chars().collect()).collect();
    let towels: Vec<String> = towels.lines().map(|x| x.chars().collect()).collect();
    patterns.sort();
    patterns.sort_by_key(|k| k.len());

    (towels, patterns)
}

#[cached(
    ty = "SizedCache<String, usize>",
    create = "{ SizedCache::with_size(1000000) }",
    convert = r#"{ towel.to_owned() }"#
)]
fn dfs<'a>(towel: &'a str, patterns: &'a [String]) -> usize {
    if towel.is_empty() {
        return 1;
    }
    let mut result = vec![];
    for pattern in patterns {
        if let Some(prefix) = towel.strip_prefix(pattern) {
            result.push(dfs(prefix, patterns));
        }
    }
    result.iter().sum()
}

fn part_1(input: &str) -> usize {
    let (towels, patterns) = parse(input);

    let mut count = 0;
    for towel in towels {
        if dfs(&towel, &patterns) > 0 {
            count += 1;
        }
    }
    count
}

fn part_2(input: &str) -> usize {
    let (towels, patterns) = parse(input);

    let mut count = 0;
    for towel in towels {
        count += dfs(&towel, &patterns);
    }
    count
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
        assert_eq!(parse(EXAMPLE), (vec![], vec![]));
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EXAMPLE), 0);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(EXAMPLE), 0);
    }
}
