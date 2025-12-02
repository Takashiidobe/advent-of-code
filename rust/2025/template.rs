#[allow(dead_code)]
const EXAMPLE: &str = include_str!("example.txt");
#[allow(dead_code)]
const INPUT: &str = include_str!("input.txt");

fn parse(input: &str) -> Vec<&str> {
    todo!()
}

fn part_1(input: &str) -> i64 {
    todo!()
}

fn part_2(input: &str) -> i64 {
    todo!()
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
        assert_eq!(parse(EXAMPLE), vec!["input"]);
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
