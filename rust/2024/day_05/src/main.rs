use std::collections::{HashMap, HashSet, VecDeque};

#[allow(dead_code)]
const EXAMPLE_INPUT: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

fn parse(input: &str) -> (Vec<(i64, i64)>, Vec<Vec<i64>>) {
    let split: Vec<&str> = input.split("\n\n").collect();
    let (rules, pages) = (split[0], split[1]);

    let rule_lines: Vec<&str> = rules.lines().collect();
    let mut parsed_rules = vec![];
    for line in rule_lines {
        let split: Vec<i64> = line.split('|').map(|x| x.parse().unwrap()).collect();
        parsed_rules.push((split[0], split[1]));
    }

    let page_lines: Vec<&str> = pages.lines().collect();
    let mut parsed_pages = vec![];
    for line in page_lines {
        let split: Vec<i64> = line.split(',').map(|x| x.parse().unwrap()).collect();
        parsed_pages.push(split);
    }

    (parsed_rules, parsed_pages)
}

fn is_valid(page: &[i64], rules: &[(i64, i64)]) -> bool {
    let lookup: HashMap<i64, usize> =
        HashMap::from_iter(page.iter().enumerate().map(|(i, num)| (*num, i)));

    for (before, after) in rules {
        if let (Some(x), Some(y)) = (lookup.get(before), lookup.get(after)) {
            if x > y {
                return false;
            }
        }
    }
    true
}

fn kahns(page: &[i64], rules: &[(i64, i64)]) -> Vec<i64> {
    let nodes: HashSet<i64> = page.iter().copied().collect();

    let mut adj: HashMap<i64, Vec<i64>> = HashMap::new();
    let mut indeg: HashMap<i64, usize> = HashMap::new();

    for &n in &nodes {
        adj.entry(n).or_default();
        indeg.entry(n).or_insert(0);
    }

    for &(before, after) in rules {
        if nodes.contains(&before) && nodes.contains(&after) {
            adj.entry(before).or_default().push(after);
            *indeg.entry(after).or_insert(0) += 1;
        }
    }

    let mut q = VecDeque::new();
    for (&n, &d) in &indeg {
        if d == 0 {
            q.push_back(n);
        }
    }

    let mut result = Vec::with_capacity(nodes.len());
    while let Some(n) = q.pop_front() {
        result.push(n);
        if let Some(neighs) = adj.get(&n) {
            for &m in neighs {
                let d = indeg.get_mut(&m).unwrap();
                *d -= 1;
                if *d == 0 {
                    q.push_back(m);
                }
            }
        }
    }

    result
}

fn part_1(input: &str) -> i64 {
    let (rules, pages) = parse(input);
    let mut total = 0;

    for page in pages {
        if is_valid(&page, &rules) {
            total += page[page.len() / 2];
        }
    }

    total
}

fn part_2(input: &str) -> i64 {
    let (rules, pages) = parse(input);
    let mut total = 0;

    for page in pages {
        if is_valid(&page, &rules) {
            continue;
        }

        let fixed = kahns(&page, &rules);
        total += fixed[fixed.len() / 2];
    }

    total
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
        assert_eq!(
            parse(EXAMPLE_INPUT),
            (
                vec![
                    (47, 53),
                    (97, 13),
                    (97, 61),
                    (97, 47),
                    (75, 29),
                    (61, 13),
                    (75, 53),
                    (29, 13),
                    (97, 29),
                    (53, 29),
                    (61, 53),
                    (97, 53),
                    (61, 29),
                    (47, 13),
                    (75, 47),
                    (97, 75),
                    (47, 61),
                    (75, 61),
                    (47, 29),
                    (75, 13),
                    (53, 13)
                ],
                vec![
                    vec![75, 47, 61, 53, 29],
                    vec![97, 61, 53, 29, 13],
                    vec![75, 29, 13],
                    vec![75, 97, 47, 61, 53],
                    vec![61, 13, 29],
                    vec![97, 13, 75, 29, 47]
                ]
            )
        );
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EXAMPLE_INPUT), 143);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(EXAMPLE_INPUT), 123);
    }
}
