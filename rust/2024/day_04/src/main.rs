const EXAMPLE_INPUT: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn part_1(input: &str) -> u64 {
    let input = parse(input);
    let (m, n) = (input.len(), input[0].len());
    let mut count = 0;

    let rest = ['M', 'A', 'S'];

    for y in 0..m {
        for x in 0..n {
            if input[y][x] != 'X' {
                continue;
            }
            if x > 2 && input[y][x - 3..x] == ['S', 'A', 'M'] {
                count += 1;
            }
            if x < m - 3 && input[y][x + 1..x + 4] == rest {
                count += 1;
            }
            if y > 2 && (1..4).all(|k| input[y - k][x] == rest[k - 1]) {
                count += 1;
            }
            if y < m - 3 && (1..4).all(|k| input[y + k][x] == rest[k - 1]) {
                count += 1;
            }
            if y > 2 && x > 2 && (1..4).all(|k| input[y - k][x - k] == rest[k - 1]) {
                count += 1;
            }
            if y < n - 3 && x > 2 && (1..4).all(|k| input[y + k][x - k] == rest[k - 1]) {
                count += 1;
            }
            if y > 2 && x < m - 3 && (1..4).all(|k| input[y - k][x + k] == rest[k - 1]) {
                count += 1;
            }
            if y < n - 3 && x < m - 3 && (1..4).all(|k| input[y + k][x + k] == rest[k - 1]) {
                count += 1;
            }
        }
    }
    count
}

fn part_2(input: &str) -> u64 {
    let input = parse(input);

    let (m, n) = (input.len(), input[0].len());

    let mut count = 0;

    for y in 1..m - 1 {
        for x in 1..n - 1 {
            if input[y][x] == 'A' {
                continue;
            }

            if (input[y - 1][x - 1] == 'M' && input[y + 1][x + 1] == 'S'
                || input[y - 1][x - 1] == 'S' && input[y + 1][x + 1] == 'M')
                && (input[y - 1][x + 1] == 'M' && input[y + 1][x - 1] == 'S'
                    || input[y - 1][x + 1] == 'S' && input[y + 1][x - 1] == 'M')
            {
                count += 1;
            }
        }
    }
    count
}

fn main() {
    println!("Part 1 Example: {}", part_1(EXAMPLE_INPUT));
    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2 Example: {}", part_2(EXAMPLE_INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {
        assert_eq!(
            parse(EXAMPLE_INPUT),
            vec![
                vec!['M', 'M', 'M', 'S', 'X', 'X', 'M', 'A', 'S', 'M'],
                vec!['M', 'S', 'A', 'M', 'X', 'M', 'S', 'M', 'S', 'A'],
                vec!['A', 'M', 'X', 'S', 'X', 'M', 'A', 'A', 'M', 'M'],
                vec!['M', 'S', 'A', 'M', 'A', 'S', 'M', 'S', 'M', 'X'],
                vec!['X', 'M', 'A', 'S', 'A', 'M', 'X', 'A', 'M', 'M'],
                vec!['X', 'X', 'A', 'M', 'M', 'X', 'X', 'A', 'M', 'A'],
                vec!['S', 'M', 'S', 'M', 'S', 'A', 'S', 'X', 'S', 'S'],
                vec!['S', 'A', 'X', 'A', 'M', 'A', 'S', 'A', 'A', 'A'],
                vec!['M', 'A', 'M', 'M', 'M', 'X', 'M', 'M', 'M', 'M'],
                vec!['M', 'X', 'M', 'X', 'A', 'X', 'M', 'A', 'S', 'X']
            ]
        );
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EXAMPLE_INPUT), 18);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(EXAMPLE_INPUT), 9);
    }
}
