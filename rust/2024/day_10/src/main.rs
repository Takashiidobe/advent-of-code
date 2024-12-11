use std::collections::HashSet;

const INPUT: &str = include_str!("input.txt");

fn parse(input: &str) -> Vec<Vec<u8>> {
    let mut res = vec![];
    for line in input.lines() {
        let chars = line.bytes().map(|x| x - b'0').collect();
        res.push(chars);
    }

    res
}

fn dfs(
    grid: &[Vec<u8>],
    y: i64,
    x: i64,
    reachable: &mut HashSet<(i64, i64)>,
    path: Vec<u8>,
) -> u64 {
    if y < 0 || y as usize >= grid.len() || x < 0 || x as usize >= grid[0].len() {
        return 0;
    }

    if let Some(last_val) = path.last() {
        if grid[y as usize][x as usize] != last_val + 1 {
            return 0;
        }
        if grid[y as usize][x as usize] == 9 && *last_val == 8 {
            reachable.insert((y, x));
            return 1;
        }
    }

    let mut new_path = path.clone();
    new_path.push(grid[y as usize][x as usize]);

    let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    dirs.map(|(dy, dx)| dfs(grid, y + dy, x + dx, reachable, new_path.clone()))
        .iter()
        .sum()
}

fn part_1(input: &str) -> u64 {
    let grid = parse(input);
    let (m, n) = (grid.len(), grid[0].len());
    let mut reachable = HashSet::new();
    let mut res = 0;

    for y in 0..m {
        for x in 0..n {
            if grid[y][x] == 0 {
                reachable.clear();
                dfs(&grid, y as i64, x as i64, &mut reachable, vec![]);
                res += reachable.len() as u64;
            }
        }
    }

    res
}

fn part_2(input: &str) -> u64 {
    let grid = parse(input);
    let mut score = 0;
    let (m, n) = (grid.len(), grid[0].len());

    for y in 0..m {
        for x in 0..n {
            if grid[y][x] == 0 {
                score += dfs(&grid, y as i64, x as i64, &mut HashSet::new(), vec![]);
            }
        }
    }

    score
}

fn main() {
    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

#[cfg(test)]
mod tests {
    const EXAMPLE_INPUT: &str = include_str!("example.txt");
    use super::*;

    #[test]
    fn parse_test() {
        assert_eq!(
            parse(EXAMPLE_INPUT),
            vec![
                vec![8, 9, 0, 1, 0, 1, 2, 3],
                vec![7, 8, 1, 2, 1, 8, 7, 4],
                vec![8, 7, 4, 3, 0, 9, 6, 5],
                vec![9, 6, 5, 4, 9, 8, 7, 4],
                vec![4, 5, 6, 7, 8, 9, 0, 3],
                vec![3, 2, 0, 1, 9, 0, 1, 2],
                vec![0, 1, 3, 2, 9, 8, 0, 1],
                vec![1, 0, 4, 5, 6, 7, 3, 2]
            ]
        );
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EXAMPLE_INPUT), 36);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(EXAMPLE_INPUT), 81);
    }
}
