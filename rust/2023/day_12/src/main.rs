use cache_macro::cache;
use lru_cache::LruCache;

fn parse(input: &str) -> Vec<(Vec<char>, Vec<usize>)> {
    let mut v = vec![];
    for line in input.lines() {
        let split_line: Vec<_> = line.split_whitespace().collect();
        let counts: Vec<usize> = split_line[1]
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect();
        v.push((split_line[0].chars().collect(), counts));
    }
    v
}

fn parse_pt_2(input: &str) -> Vec<(Vec<char>, Vec<usize>)> {
    let mut v = vec![];
    for line in input.lines() {
        let split_line: Vec<_> = line.split_whitespace().collect();
        let extended_pattern = {
            let mut extended = String::new();
            for _ in 0..5 {
                extended.push_str(split_line[0]);
                extended.push('?');
            }
            extended.pop();
            extended.chars().collect()
        };

        let counts: Vec<usize> = split_line[1]
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect();
        let mut extended_counts = vec![];
        for _ in 0..5 {
            extended_counts.extend_from_slice(&counts);
        }
        v.push((extended_pattern, extended_counts));
    }
    v
}

#[cache(LruCache : LruCache::new(10000000))]
fn count_paths(pattern: &Vec<char>, counts: &Vec<usize>) -> usize {
    let mut result = 0;
    if pattern.is_empty() {
        return if counts.is_empty() { 1 } else { 0 };
    }
    if counts.is_empty() {
        return if !pattern.contains(&'#') { 1 } else { 0 };
    }

    if pattern[0] == '.' || pattern[0] == '?' {
        result += count_paths(&pattern[1..].to_vec(), counts);
    }

    if (pattern[0] == '#' || pattern[0] == '?')
        && counts[0] <= pattern.len()
        && !pattern[..counts[0]].contains(&'.')
        && (counts[0] == pattern.len() || pattern[counts[0]] != '#')
    {
        if counts.is_empty() && counts[0] >= pattern.len() {
            result += count_paths(&vec![], &vec![]);
        } else if counts.is_empty() {
            result += count_paths(&pattern[counts[0] + 1..].to_vec(), &vec![]);
        } else if counts[0] >= pattern.len() {
            result += count_paths(&vec![], &counts[1..].to_vec());
        } else {
            result += count_paths(&pattern[counts[0] + 1..].to_vec(), &counts[1..].to_vec());
        }
    }

    result
}

fn part_1(input: &str) -> usize {
    let mut total = 0;
    let parsed_input = parse(input);

    for (pattern, counts) in parsed_input {
        total += count_paths(&pattern, &counts);
    }
    total
}

fn part_2(input: &str) -> usize {
    let mut total = 0;
    let parsed_input = parse_pt_2(input);

    for (pattern, counts) in parsed_input {
        total += count_paths(&pattern, &counts);
    }
    total
}

fn main() {
    let input = include_str!("../input.txt");
    println!("part 1: {}, part 2: {}", part_1(input), part_2(input));
}

#[test]
fn example_1() {
    let example = include_str!("../example.txt");
    assert_eq!(part_1(example), 21);
}

#[test]
fn example_2() {
    let example = include_str!("../example.txt");
    assert_eq!(part_2(example), 525152);
}
