#[allow(dead_code)]
const EXAMPLE: &str = include_str!("example.txt");
#[allow(dead_code)]
const INPUT: &str = include_str!("input.txt");

fn parse(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|l| l.bytes().collect()).collect()
}

const DIRECTIONS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn inbounds(m: i32, n: i32, y: i32, x: i32) -> bool {
    0 <= y && y < m && 0 <= x && x < n
}

fn neighbor_count(grid: &[Vec<u8>], y: i32, x: i32) -> u8 {
    let (m, n) = (grid.len() as i32, grid[0].len() as i32);
    let mut neighbors = 0;
    for (dy, dx) in DIRECTIONS {
        let (new_y, new_x) = (dy + y, dx + x);
        if inbounds(m, n, new_y, new_x) && grid[new_y as usize][new_x as usize] == b'@' {
            neighbors += 1;
        }
    }
    neighbors
}

fn part_1(input: &str) -> i64 {
    let grid = parse(input);
    let (m, n) = (grid.len(), grid[0].len());
    let mut accessible = 0;

    for y in 0..m {
        for x in 0..n {
            if grid[y][x] == b'@' && neighbor_count(&grid, y as i32, x as i32) < 4 {
                accessible += 1;
            }
        }
    }

    accessible
}

fn part_2(input: &str) -> i64 {
    let mut grid = parse(input);
    let (m, n) = (grid.len(), grid[0].len());
    let mut accessible = 0;

    loop {
        let prev_accessible = accessible;
        for y in 0..m {
            for x in 0..n {
                if grid[y][x] == b'@' && neighbor_count(&grid, y as i32, x as i32) < 4 {
                    accessible += 1;
                    grid[y][x] = b'.';
                }
            }
        }
        if accessible == prev_accessible {
            break;
        }
    }

    accessible
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
                vec![b'.', b'.', b'@', b'@', b'.', b'@', b'@', b'@', b'@', b'.'],
                vec![b'@', b'@', b'@', b'.', b'@', b'.', b'@', b'.', b'@', b'@'],
                vec![b'@', b'@', b'@', b'@', b'@', b'.', b'@', b'.', b'@', b'@'],
                vec![b'@', b'.', b'@', b'@', b'@', b'@', b'.', b'.', b'@', b'.'],
                vec![b'@', b'@', b'.', b'@', b'@', b'@', b'@', b'.', b'@', b'@'],
                vec![b'.', b'@', b'@', b'@', b'@', b'@', b'@', b'@', b'.', b'@'],
                vec![b'.', b'@', b'.', b'@', b'.', b'@', b'.', b'@', b'@', b'@'],
                vec![b'@', b'.', b'@', b'@', b'@', b'.', b'@', b'@', b'@', b'@'],
                vec![b'.', b'@', b'@', b'@', b'@', b'@', b'@', b'@', b'@', b'.'],
                vec![b'@', b'.', b'@', b'.', b'@', b'@', b'@', b'.', b'@', b'.']
            ]
        );
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EXAMPLE), 13);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(EXAMPLE), 43);
    }
}
