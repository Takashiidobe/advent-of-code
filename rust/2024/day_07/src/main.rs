const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

fn parse(input: &str) -> Vec<(i64, Vec<i64>)> {
    let mut res = vec![];
    for line in input.lines() {
        let split: Vec<&str> = line.split(": ").collect();
        let (test_val, numbers) = (split[0], split[1]);
        let parsed_numbers = numbers
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        res.push((test_val.parse().unwrap(), parsed_numbers));
    }

    res
}

fn concat(a: i64, b: i64) -> i64 {
    let mut pow = 1_i64;
    let mut t = b;
    loop {
        pow *= 10;
        t /= 10;
        if t == 0 {
            break;
        }
    }
    a * pow + b
}

fn dfs(nums: &[i64], idx: usize, acc: i64, expected: i64) -> bool {
    if idx == nums.len() {
        return acc == expected;
    }

    let next = nums[idx];
    let plus = dfs(nums, idx + 1, acc + next, expected);
    let mul = dfs(nums, idx + 1, acc * next, expected);
    plus || mul
}

fn dfs2(nums: &[i64], idx: usize, acc: i64, expected: i64) -> bool {
    if idx == nums.len() {
        return acc == expected;
    }

    let next = nums[idx];
    let plus = dfs2(nums, idx + 1, acc + next, expected);
    let mul = dfs2(nums, idx + 1, acc * next, expected);
    let cat = dfs2(nums, idx + 1, concat(acc, next), expected);
    plus || mul || cat
}

fn part_1(input: &str) -> i64 {
    let parsed = parse(input);
    let mut total = 0;
    for (expected, nums) in parsed {
        if dfs(&nums, 1, nums[0], expected) {
            total += expected;
        }
    }
    total
}

fn part_2(input: &str) -> i64 {
    let parsed = parse(input);
    let mut total = 0;
    for (expected, nums) in parsed {
        if dfs2(&nums, 1, nums[0], expected) {
            total += expected;
        }
    }
    total
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
            vec![
                (190, vec![10, 19]),
                (3267, vec![81, 40, 27]),
                (83, vec![17, 5]),
                (156, vec![15, 6]),
                (7290, vec![6, 8, 6, 15]),
                (161011, vec![16, 10, 13]),
                (192, vec![17, 8, 14]),
                (21037, vec![9, 7, 18, 13]),
                (292, vec![11, 6, 16, 20])
            ]
        );
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EXAMPLE), 3749);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(EXAMPLE), 11387);
    }
}
