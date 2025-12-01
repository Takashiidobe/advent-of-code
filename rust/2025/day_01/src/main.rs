const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

#[derive(Debug, Clone, Copy, PartialEq)]
enum Dir {
    Left(i64),
    Right(i64),
}

fn parse(input: &str) -> Vec<Dir> {
    let mut v = vec![];
    for line in input.lines() {
        let str_num: String = line.chars().skip(1).collect();
        let num: i64 = str_num.parse().unwrap();

        if line.starts_with('L') {
            v.push(Dir::Left(num));
        } else {
            v.push(Dir::Right(num));
        }
    }

    v
}

fn part_1(input: &str) -> i64 {
    let input = parse(input);
    let mut curr = 50;
    let mut count = 0;
    for dir in input {
        match dir {
            Dir::Left(n) => curr -= n,
            Dir::Right(n) => curr += n,
        }
        curr = ((curr % 100) + 100) % 100;
        if curr == 0 {
            count += 1;
        }
    }
    count
}

fn part_2(input: &str) -> i64 {
    let input = parse(input);
    let mut curr = 50;
    let mut count = 0;
    for dir in input {
        let magnitude = match dir {
            Dir::Left(n) | Dir::Right(n) => n,
        };

        for _ in 0..magnitude {
            if matches!(dir, Dir::Left(_)) {
                curr -= 1;
            } else {
                curr += 1;
            }
            curr = ((curr % 100) + 100) % 100;
            if curr == 0 {
                count += 1;
            }
        }
    }
    count
}

fn main() {
    println!("Example Part 1: {}", part_1(EXAMPLE));
    println!("Part 1: {}", part_1(INPUT));
    println!("Example Part 2: {}", part_2(EXAMPLE));
    println!("Part 2: {}", part_2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {
        use Dir::*;
        let dirs = vec![
            Left(68),
            Left(30),
            Right(48),
            Left(5),
            Right(60),
            Left(55),
            Left(1),
            Left(99),
            Right(14),
            Left(82),
        ];
        assert_eq!(parse(EXAMPLE), dirs);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EXAMPLE), 3);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(EXAMPLE), 6);
    }
}
