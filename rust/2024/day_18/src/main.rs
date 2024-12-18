use std::collections::{HashSet, VecDeque};

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

fn parse(input: &str, count: usize) -> Vec<(isize, isize)> {
    let mut res = vec![];
    for (i, line) in input.lines().enumerate() {
        if i == count {
            break;
        }
        let split: Vec<_> = line.split(',').collect();
        res.push((split[0].parse().unwrap(), split[1].parse().unwrap()));
    }
    res
}

fn part_1(input: &str, height: isize, width: isize, count: usize) -> Option<i64> {
    let parsed = parse(input, count);

    let mut grid = vec![vec!['.'; height as usize]; width as usize];

    for (x, y) in parsed {
        grid[y as usize][x as usize] = '#';
    }

    let start = (0, 0);
    let end = (height - 1, width - 1);

    let mut q = VecDeque::from([(start, 0)]);
    let mut visited = HashSet::new();

    let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    while let Some((curr, steps)) = q.pop_front() {
        let (y, x) = curr;
        if y < 0 || y >= height || x < 0 || x >= width {
            continue;
        }
        if grid[y as usize][x as usize] == '#' {
            continue;
        }
        if curr == end {
            return Some(steps);
        }
        if visited.contains(&curr) {
            continue;
        }
        visited.insert(curr);

        for (dy, dx) in dirs {
            q.push_back(((dy + y, dx + x), steps + 1));
        }
    }

    None
}

fn part_2(input: &str, height: isize, width: isize, skip: usize) -> Option<(isize, isize)> {
    let parsed = parse(input, usize::MAX);

    for (i, _) in parsed.iter().enumerate().skip(skip) {
        if part_1(input, height, width, i).is_none() {
            return Some(parsed[i - 1]);
        }
    }
    None
}

fn main() {
    println!("Part 1 Example: {}", part_1(EXAMPLE, 7, 7, 12).unwrap());
    println!("Part 1: {}", part_1(INPUT, 71, 71, 1024).unwrap());
    println!("Part 2 Example: {:?}", part_2(EXAMPLE, 7, 7, 13));
    println!("Part 2: {:?}", part_2(INPUT, 71, 71, 1025));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {
        assert_eq!(
            parse(EXAMPLE, usize::MAX),
            vec![
                (5, 4),
                (4, 2),
                (4, 5),
                (3, 0),
                (2, 1),
                (6, 3),
                (2, 4),
                (1, 5),
                (0, 6),
                (3, 3),
                (2, 6),
                (5, 1),
                (1, 2),
                (5, 5),
                (2, 5),
                (6, 5),
                (1, 4),
                (0, 4),
                (6, 4),
                (1, 1),
                (6, 1),
                (1, 0),
                (0, 5),
                (1, 6),
                (2, 0)
            ]
        );
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EXAMPLE, 7, 7, 12), Some(22));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(EXAMPLE, 7, 7, 13), Some((6, 1)));
    }
}
