#[allow(dead_code)]
const EXAMPLE: &str = include_str!("example.txt");
#[allow(dead_code)]
const INPUT: &str = include_str!("input.txt");

#[derive(Debug, Clone, PartialEq, Copy)]
enum Op {
    Plus,
    Mul,
    Num(i64),
}

impl From<&str> for Op {
    fn from(value: &str) -> Self {
        match value {
            "*" => Op::Mul,
            "+" => Op::Plus,
            _ => Op::Num(value.parse().unwrap()),
        }
    }
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

fn parse(input: &str) -> Vec<Vec<Op>> {
    let mut intermediates: Vec<Vec<&str>> = vec![];
    for line in input.lines() {
        let line = line.trim();
        intermediates.push(line.split_whitespace().collect());
    }

    let transposed = transpose(intermediates);

    let res: Vec<Vec<Op>> = transposed
        .iter()
        .map(|l| l.iter().map(|i| Op::from(*i)).collect())
        .collect();

    res
}

fn part_1(input: &str) -> i64 {
    let parsed = parse(input);
    let mut total = 0;
    for mut problem in parsed {
        let op = problem.pop().unwrap();
        let line_total = match op {
            Op::Plus => problem.iter().fold(0, |acc, op| {
                if let Op::Num(n) = op {
                    acc + n
                } else {
                    unreachable!()
                }
            }),
            Op::Mul => problem.iter().fold(1, |acc, op| {
                if let Op::Num(n) = op {
                    acc * n
                } else {
                    unreachable!()
                }
            }),
            Op::Num(_) => unreachable!(),
        };
        total += line_total;
    }
    total
}

fn parse2(input: &str) -> Vec<Vec<char>> {
    let mut intermediates: Vec<Vec<char>> = vec![];
    for line in input.lines() {
        intermediates.push(line.chars().collect());
    }

    transpose(intermediates)
}

fn part_2(input: &str) -> i64 {
    let parsed = parse2(input);
    let mut total = 0;
    let mut curr_line = 0;
    let mut curr_op = Op::Mul;

    for line in parsed {
        if line.iter().all(char::is_ascii_whitespace) {
            total += curr_line;
            curr_line = 0;
            continue;
        }

        let last = line.last().unwrap();
        match last {
            '+' => curr_op = Op::Plus,
            '*' => curr_op = Op::Mul,
            _ => {}
        }

        let remaining_chars: String = line[..line.len() - 1].iter().collect();
        let trimmed = remaining_chars.trim();

        let curr: i64 = trimmed.parse().unwrap();

        match curr_op {
            Op::Plus => curr_line += curr,
            Op::Mul => {
                if curr_line == 0 {
                    curr_line = curr
                } else {
                    curr_line *= curr
                }
            }
            _ => unreachable!(),
        }
    }

    // get the last line
    total += curr_line;

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
        use Op::*;
        assert_eq!(
            parse(EXAMPLE),
            vec![
                vec![Num(123), Num(45), Num(6), Mul],
                vec![Num(328), Num(64), Num(98), Plus],
                vec![Num(51), Num(387), Num(215), Mul],
                vec![Num(64), Num(23), Num(314), Plus]
            ]
        );
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EXAMPLE), 4277556);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(EXAMPLE), 3263827);
    }
}
