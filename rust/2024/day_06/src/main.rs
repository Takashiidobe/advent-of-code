use std::collections::HashSet;

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn parse(input: &str) -> (Vec<Vec<u8>>, (i32, i32)) {
    let mut res = vec![];
    let mut pos = None;
    for (i, line) in input.lines().enumerate() {
        let bytes: Vec<_> = line.bytes().collect();
        let x = bytes.iter().position(|n| *n == b'^');
        if let Some(x_pos) = x {
            pos = Some((i as i32, x_pos as i32));
        }
        res.push(bytes);
    }
    (res, pos.unwrap())
}

fn visited(grid: &[Vec<u8>], y: i32, x: i32) -> HashSet<(i32, i32)> {
    let (mut y, mut x) = (y, x);
    let (m, n) = (grid.len() as i32, grid[0].len() as i32);
    let mut dir = 0;
    let mut visited = HashSet::new();

    loop {
        if y >= m || y < 0 || x >= n || x < 0 {
            break;
        }
        let (dy, dx) = DIRECTIONS[dir];
        if grid[y as usize][x as usize] == b'#' {
            y -= dy;
            x -= dx;
            dir = (dir + 1) % DIRECTIONS.len();
        } else {
            visited.insert((y, x));
            y += dy;
            x += dx;
        }
    }

    visited
}

fn loop_check(grid: &[Vec<u8>], y: i32, x: i32) -> bool {
    let (mut y, mut x) = (y, x);
    let (m, n) = (grid.len() as i32, grid[0].len() as i32);
    let mut dir = 0;
    let mut visited = HashSet::new();

    loop {
        if y >= m || y < 0 || x >= n || x < 0 {
            break;
        }
        if visited.contains(&(y, x, DIRECTIONS[dir])) {
            return true;
        }
        let (dy, dx) = DIRECTIONS[dir];
        if grid[y as usize][x as usize] == b'#' {
            y -= dy;
            x -= dx;
            dir = (dir + 1) % DIRECTIONS.len();
        } else {
            visited.insert((y, x, DIRECTIONS[dir]));
            y += dy;
            x += dx;
        }
    }

    false // escapes
}

fn part_1(input: &str) -> usize {
    let (grid, (y, x)) = parse(input);
    visited(&grid, y, x).len()
}

// we can simulate the guard by marking every visited as
fn part_2(input: &str) -> usize {
    let (mut grid, (start_y, start_x)) = parse(input);
    let visited = visited(&grid, start_y, start_x);
    let mut obstacles = 0;

    for (y, x) in visited {
        if start_y == y && start_x == x {
            continue;
        }
        grid[y as usize][x as usize] = b'#';
        if loop_check(&grid, start_y, start_x) {
            obstacles += 1;
        }
        grid[y as usize][x as usize] = b'.';
    }

    obstacles
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
    const EXAMPLE_INPUT: &str = include_str!("example.txt");

    #[test]
    fn parse_test() {
        assert_eq!(
            parse(EXAMPLE_INPUT),
            (
                vec![
                    vec![b'.', b'.', b'.', b'.', b'#', b'.', b'.', b'.', b'.', b'.'],
                    vec![b'.', b'.', b'.', b'.', b'.', b'.', b'.', b'.', b'.', b'#'],
                    vec![b'.', b'.', b'.', b'.', b'.', b'.', b'.', b'.', b'.', b'.'],
                    vec![b'.', b'.', b'#', b'.', b'.', b'.', b'.', b'.', b'.', b'.'],
                    vec![b'.', b'.', b'.', b'.', b'.', b'.', b'.', b'#', b'.', b'.'],
                    vec![b'.', b'.', b'.', b'.', b'.', b'.', b'.', b'.', b'.', b'.'],
                    vec![b'.', b'#', b'.', b'.', b'^', b'.', b'.', b'.', b'.', b'.'],
                    vec![b'.', b'.', b'.', b'.', b'.', b'.', b'.', b'.', b'#', b'.'],
                    vec![b'#', b'.', b'.', b'.', b'.', b'.', b'.', b'.', b'.', b'.'],
                    vec![b'.', b'.', b'.', b'.', b'.', b'.', b'#', b'.', b'.', b'.']
                ],
                (6, 4)
            )
        );
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EXAMPLE_INPUT), 41);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(EXAMPLE_INPUT), 6);
    }
}
