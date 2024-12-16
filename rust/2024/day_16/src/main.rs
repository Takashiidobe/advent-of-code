use std::{
    cmp::Reverse,
    collections::{BTreeMap, BinaryHeap, HashMap, HashSet},
};

#[allow(dead_code)]
const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

#[derive(Debug, Default, Clone, Copy, PartialEq)]
enum Tile {
    Wall,
    #[default]
    Open,
}

#[derive(Debug, Default, Clone, PartialEq)]
struct Maze {
    start: (isize, isize),
    end: (isize, isize),
    map: Vec<Vec<Tile>>,
}

fn parse(input: &str) -> Maze {
    let mut map = vec![];
    let mut start = None;
    let mut end = None;
    for (y, line) in input.lines().enumerate() {
        let mut curr = vec![];
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => curr.push(Tile::Wall),
                '.' => curr.push(Tile::Open),
                'S' => {
                    start = Some((y as isize, x as isize));
                    curr.push(Tile::Open);
                }
                'E' => {
                    end = Some((y as isize, x as isize));
                    curr.push(Tile::Open);
                }
                _ => unreachable!(),
            }
        }
        map.push(curr);
    }

    Maze {
        start: start.expect("No start tile"),
        end: end.expect("No end tile"),
        map,
    }
}

fn part_1(input: &str) -> usize {
    let Maze { start, end, map } = parse(input);
    let (m, n) = (map.len(), map[0].len());

    let dirs = [(1, 0), (0, -1), (0, 1), (-1, 0)];

    let mut heap = BinaryHeap::new();
    let mut visited = HashSet::new();
    heap.push((Reverse(0), start, (0, 1)));

    while let Some((Reverse(cost), (y, x), dir)) = heap.pop() {
        if y < 0 || y as usize >= m || x < 0 || x as usize >= n {
            continue;
        }
        if map[y as usize][x as usize] == Tile::Wall {
            continue;
        }
        if visited.contains(&(y, x)) {
            continue;
        }

        if (y, x) == end {
            return cost;
        }

        visited.insert((y, x));

        for (dy, dx) in dirs {
            let (new_y, new_x) = (dy + y, dx + x);
            heap.push((
                Reverse(cost + (if (dy, dx) == dir { 1 } else { 1001 })),
                (new_y, new_x),
                (dy, dx),
            ));
        }
    }

    0
}

fn part_2(input: &str) -> usize {
    let Maze { start, end, map } = parse(input);
    let (m, n) = (map.len(), map[0].len());

    let dirs = [(1, 0), (0, -1), (0, 1), (-1, 0)];

    let mut heap = BinaryHeap::new();
    let mut visited = HashMap::new();
    let mut best_paths: BTreeMap<usize, HashSet<(isize, isize)>> = BTreeMap::new();
    heap.push((Reverse(0), start, (0, 1), vec![]));

    while let Some((Reverse(cost), (y, x), dir, path)) = heap.pop() {
        if cost > *visited.get(&(y, x, dir)).unwrap_or(&usize::MAX) {
            continue;
        }
        if y < 0 || y as usize >= m || x < 0 || x as usize >= n {
            continue;
        }
        if map[y as usize][x as usize] == Tile::Wall {
            continue;
        }

        if (y, x) == end {
            best_paths.entry(cost).or_default().extend(path.into_iter());
            continue;
        }

        visited.insert((y, x, dir), cost);
        let mut cloned_path = path.clone();
        cloned_path.push((y, x));

        for (dy, dx) in dirs {
            let (new_y, new_x) = (dy + y, dx + x);
            heap.push((
                Reverse(cost + (if (dy, dx) == dir { 1 } else { 1001 })),
                (new_y, new_x),
                (dy, dx),
                cloned_path.clone(),
            ));
        }
    }

    best_paths.pop_first().unwrap().1.len() + 1
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
    use Tile::*;

    #[test]
    fn parse_test() {
        assert_eq!(
            parse(EXAMPLE),
            Maze {
                start: (13, 1),
                end: (1, 13),
                map: vec![
                    vec![
                        Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall,
                        Wall, Wall, Wall
                    ],
                    vec![
                        Wall, Open, Open, Open, Open, Open, Open, Open, Wall, Open, Open, Open,
                        Open, Open, Wall
                    ],
                    vec![
                        Wall, Open, Wall, Open, Wall, Wall, Wall, Open, Wall, Open, Wall, Wall,
                        Wall, Open, Wall
                    ],
                    vec![
                        Wall, Open, Open, Open, Open, Open, Wall, Open, Wall, Open, Open, Open,
                        Wall, Open, Wall
                    ],
                    vec![
                        Wall, Open, Wall, Wall, Wall, Open, Wall, Wall, Wall, Wall, Wall, Open,
                        Wall, Open, Wall
                    ],
                    vec![
                        Wall, Open, Wall, Open, Wall, Open, Open, Open, Open, Open, Open, Open,
                        Wall, Open, Wall
                    ],
                    vec![
                        Wall, Open, Wall, Open, Wall, Wall, Wall, Wall, Wall, Open, Wall, Wall,
                        Wall, Open, Wall
                    ],
                    vec![
                        Wall, Open, Open, Open, Open, Open, Open, Open, Open, Open, Open, Open,
                        Wall, Open, Wall
                    ],
                    vec![
                        Wall, Wall, Wall, Open, Wall, Open, Wall, Wall, Wall, Wall, Wall, Open,
                        Wall, Open, Wall
                    ],
                    vec![
                        Wall, Open, Open, Open, Wall, Open, Open, Open, Open, Open, Wall, Open,
                        Wall, Open, Wall
                    ],
                    vec![
                        Wall, Open, Wall, Open, Wall, Open, Wall, Wall, Wall, Open, Wall, Open,
                        Wall, Open, Wall
                    ],
                    vec![
                        Wall, Open, Open, Open, Open, Open, Wall, Open, Open, Open, Wall, Open,
                        Wall, Open, Wall
                    ],
                    vec![
                        Wall, Open, Wall, Wall, Wall, Open, Wall, Open, Wall, Open, Wall, Open,
                        Wall, Open, Wall
                    ],
                    vec![
                        Wall, Open, Open, Open, Wall, Open, Open, Open, Open, Open, Wall, Open,
                        Open, Open, Wall
                    ],
                    vec![
                        Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall,
                        Wall, Wall, Wall
                    ]
                ]
            }
        )
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EXAMPLE), 7036);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(EXAMPLE), 45);
    }
}
