const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

fn parse(input: &str) -> Vec<(i64, i64)> {
    let input = input.trim();
    let range: Vec<&str> = input.split(',').collect();
    let mut output = vec![];
    for r in range {
        let (start, end) = r.split_once('-').unwrap();
        dbg!(start, end);
        let (start, end): (i64, i64) = (start.parse().unwrap(), end.parse().unwrap());
        output.push((start, end));
    }
    output
}

fn is_repeated_once(input: i64) -> bool {
    let s: Vec<_> = input.to_string().chars().collect();
    let (start, end) = s.split_at(s.len() / 2);
    start == end
}

fn part_1(input: &str) -> i64 {
    let parsed = parse(input);
    let mut count = 0;

    for (start, end) in parsed {
        for i in start..=end {
            if is_repeated_once(i) {
                count += i;
            }
        }
    }

    count
}

// now we need to find when it's repeated at least twice
fn is_repeated_at_least_twice(input: i64) -> bool {
    let s: Vec<_> = input.to_string().chars().collect();
    for i in 1..=(s.len() / 2) {
        let pat = &s[..i];
        if pat.repeat(s.len() / i) == s {
            return true;
        }
    }
    false
}

fn part_2(input: &str) -> i64 {
    let parsed = parse(input);
    let mut count = 0;

    for (start, end) in parsed {
        for i in start..=end {
            if is_repeated_at_least_twice(i) {
                count += i;
            }
        }
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
        assert_eq!(
            parse(EXAMPLE),
            vec![
                (11, 22),
                (95, 115),
                (998, 1012),
                (1188511880, 1188511890),
                (222220, 222224),
                (1698522, 1698528),
                (446443, 446449),
                (38593856, 38593862),
                (565653, 565659),
                (824824821, 824824827),
                (2121212118, 2121212124)
            ]
        );
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EXAMPLE), 1227775554);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(EXAMPLE), 4174379265);
    }
}
