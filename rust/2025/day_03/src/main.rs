#[allow(dead_code)]
const EXAMPLE: &str = include_str!("example.txt");
#[allow(dead_code)]
const INPUT: &str = include_str!("input.txt");

fn parse(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|l| l.bytes().map(|b| (b - b'0') as i64).collect())
        .collect()
}

fn part_1(input: &str) -> i64 {
    let parsed = parse(input);
    let mut res = 0;

    for bank in parsed {
        let mut stack = vec![];
        let mut max_so_far = 0;
        for battery in bank {
            stack.push(battery);
            if stack.len() < 3 {
                continue;
            }

            if stack[1] > stack[0] {
                stack.remove(0);
            } else if stack[2] > stack[1] {
                stack.remove(1);
            } else {
                stack.pop();
            }
        }
        let curr: i64 = stack[0] * 10 + stack[1];
        max_so_far = max_so_far.max(curr);
        res += max_so_far;
    }

    res
}

fn part_2(input: &str) -> i64 {
    let parsed = parse(input);
    let mut res = 0;

    for bank in parsed {
        let mut stack = vec![];
        let mut max_so_far = 0;
        for battery in bank {
            stack.push(battery);
            if stack.len() < 13 {
                continue;
            }

            let mut index = 12;
            for (i, (curr, next)) in stack.iter().zip(stack.iter().skip(1)).enumerate() {
                if next > curr {
                    index = i;
                    break;
                }
            }

            stack.remove(index);
        }
        let mut curr = 0;
        for digit in &stack {
            curr = curr * 10 + digit;
        }
        max_so_far = max_so_far.max(curr);
        res += max_so_far;
    }
    res
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
                vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1],
                vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9],
                vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8],
                vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1],
            ]
        );
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EXAMPLE), 357);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(EXAMPLE), 3121910778619);
    }
}
