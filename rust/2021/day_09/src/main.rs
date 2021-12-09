use std::collections::{HashSet, VecDeque};

fn get_input() -> &'static str {
    include_str!("../input.txt").trim()
}

fn parse_input(file: &str) -> Vec<Vec<u32>> {
    let mut v = vec![];
    for line in file.lines() {
        let l: Vec<u32> = line
            .split("")
            .collect::<Vec<&str>>()
            .into_iter()
            .filter(|&x| x != "")
            .map(|x| x.parse().unwrap())
            .collect();
        v.push(l);
    }
    v
}
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Point {
    row: usize,
    col: usize,
    height: u32,
}

fn get_low_points(height_map: &[Vec<u32>]) -> Vec<Point> {
    let mut low_points = Vec::new();
    for (i, row) in height_map.iter().enumerate() {
        for (j, height) in row.iter().enumerate() {
            if get_adjacent_points(height_map, i, j)
                .iter()
                .all(|p| p.height > *height)
            {
                low_points.push(Point {
                    row: i,
                    col: j,
                    height: *height,
                });
            }
        }
    }
    low_points
}

fn part_1(input: &[Vec<u32>]) -> u32 {
    get_low_points(input).iter().map(|p| p.height + 1).sum()
}

fn get_adjacent_points(height_map: &[Vec<u32>], row: usize, col: usize) -> [Point; 4] {
    let out_of_bounds = Point {
        row: 0,
        col: 0,
        height: u32::MAX,
    };
    let left = if col == 0 {
        out_of_bounds
    } else {
        Point {
            row,
            col: col - 1,
            height: height_map[row][col - 1],
        }
    };
    let top = if row == 0 {
        out_of_bounds
    } else {
        Point {
            row: row - 1,
            col,
            height: height_map[row - 1][col],
        }
    };
    let right = if col == height_map[0].len() - 1 {
        out_of_bounds
    } else {
        Point {
            row,
            col: col + 1,
            height: height_map[row][col + 1],
        }
    };
    let bottom = if row == height_map.len() - 1 {
        out_of_bounds
    } else {
        Point {
            row: row + 1,
            col,
            height: height_map[row + 1][col],
        }
    };
    [left, top, right, bottom]
}

fn get_basin_size(height_map: &[Vec<u32>], low_point: Point) -> u32 {
    let mut queue = VecDeque::from([low_point]);
    let mut visited = HashSet::new();
    visited.insert(low_point);
    while !queue.is_empty() {
        let current_point = queue.pop_front().unwrap();
        for p in get_adjacent_points(height_map, current_point.row, current_point.col) {
            if p.height > current_point.height && p.height < 9 && !visited.contains(&p) {
                queue.push_back(p);
                visited.insert(p);
            }
        }
    }
    visited.len() as u32
}

fn part_2(input: &Vec<Vec<u32>>) -> u32 {
    let low_points = get_low_points(input);
    let mut basin_sizes: Vec<u32> = low_points
        .iter()
        .map(|p| get_basin_size(input, *p))
        .collect::<Vec<u32>>();
    basin_sizes.sort_unstable_by(|a, b| b.cmp(a));
    basin_sizes[..3].iter().product::<u32>()
}

fn main() {
    let input = parse_input(get_input());
    println!("part 1: {}", part_1(&input));
    println!("part 2: {}", part_2(&input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let test_input = r#"
2199943210
3987894921
9856789892
8767896789
9899965678
"#
        .trim();
        let input = parse_input(test_input);
        assert_eq!(part_1(&input), 15);
        assert_eq!(part_2(&input), 1134);
    }
}
