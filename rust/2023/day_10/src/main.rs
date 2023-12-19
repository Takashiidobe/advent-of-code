use std::collections::HashSet;
use Tile::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Tile {
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
    Ground,
    Start,
}

impl Tile {
    fn from(c: char) -> Self {
        match c {
            '|' => Self::NS,
            '-' => Self::EW,
            'L' => Self::NE,
            'J' => Self::NW,
            '7' => Self::SW,
            'F' => Self::SE,
            '.' => Self::Ground,
            'S' => Self::Start,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
struct Coordinate {
    x: usize,
    y: usize,
}

fn parse(input: &str) -> (Vec<Vec<Tile>>, Coordinate) {
    let mut start = Coordinate::default();
    let map = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    let tile = Tile::from(c);
                    if tile == Tile::Start {
                        start = Coordinate { x, y }
                    }
                    tile
                })
                .collect()
        })
        .collect();
    (map, start)
}

fn build_loop(start: Coordinate, map: &[Vec<Tile>]) -> HashSet<Coordinate> {
    let mut loop_coords = HashSet::new();
    loop_coords.insert(start);
    let mut to_visit = start.valid_neighbors(map);

    while let Some(curr_pos) = to_visit.pop() {
        for neighbour in curr_pos.valid_neighbors(map) {
            if !loop_coords.contains(&neighbour) {
                to_visit.push(neighbour);
                loop_coords.insert(neighbour);
            }
        }
    }

    loop_coords
}

impl Coordinate {
    fn new(y: usize, x: usize) -> Self {
        Self { y, x }
    }

    fn look_south(&self, map: &[Vec<Tile>], neighbors: &mut Vec<Coordinate>, y: usize, x: usize) {
        if y > 0 && matches!(map[y - 1][x], NS | SW | SE) {
            neighbors.push(Coordinate::new(y - 1, x));
        }
    }

    fn look_south_or_start(
        &self,
        map: &[Vec<Tile>],
        neighbors: &mut Vec<Coordinate>,
        y: usize,
        x: usize,
    ) {
        if y > 0 && matches!(map[y - 1][x], NS | SW | SE | Start) {
            neighbors.push(Coordinate::new(y - 1, x));
        }
    }

    fn look_north(&self, map: &[Vec<Tile>], neighbors: &mut Vec<Coordinate>, y: usize, x: usize) {
        if y < map.len() - 1 && matches!(map[y + 1][x], NS | NW | NE) {
            neighbors.push(Coordinate::new(y + 1, x))
        }
    }

    fn look_north_or_start(
        &self,
        map: &[Vec<Tile>],
        neighbors: &mut Vec<Coordinate>,
        y: usize,
        x: usize,
    ) {
        if y < map.len() - 1 && matches!(map[y + 1][x], NS | NW | NE | Start) {
            neighbors.push(Coordinate::new(y + 1, x))
        }
    }

    fn look_east(&self, map: &[Vec<Tile>], neighbors: &mut Vec<Coordinate>, y: usize, x: usize) {
        if x > 0 && matches!(map[y][x - 1], EW | SE | NE) {
            neighbors.push(Coordinate::new(y, x - 1))
        }
    }

    fn look_east_or_start(
        &self,
        map: &[Vec<Tile>],
        neighbors: &mut Vec<Coordinate>,
        y: usize,
        x: usize,
    ) {
        if x > 0 && matches!(map[y][x - 1], EW | SE | NE | Start) {
            neighbors.push(Coordinate::new(y, x - 1))
        }
    }

    fn look_west(&self, map: &[Vec<Tile>], neighbors: &mut Vec<Coordinate>, y: usize, x: usize) {
        if x < map[0].len() - 1 && matches!(map[y][x + 1], EW | NW | SW) {
            neighbors.push(Coordinate::new(y, x + 1))
        }
    }

    fn look_west_or_start(
        &self,
        map: &[Vec<Tile>],
        neighbors: &mut Vec<Coordinate>,
        y: usize,
        x: usize,
    ) {
        if x < map[0].len() - 1 && matches!(map[y][x + 1], EW | NW | SW | Start) {
            neighbors.push(Coordinate::new(y, x + 1))
        }
    }

    fn valid_neighbors(&self, map: &[Vec<Tile>]) -> Vec<Coordinate> {
        let mut neighbors = vec![];
        let Coordinate { x, y } = *self;

        match map[y][x] {
            Ground => (),
            Start => {
                self.look_south(map, &mut neighbors, y, x);
                self.look_north(map, &mut neighbors, y, x);
                self.look_east(map, &mut neighbors, y, x);
                self.look_west(map, &mut neighbors, y, x);
            }
            NS => {
                self.look_south_or_start(map, &mut neighbors, y, x);
                self.look_north_or_start(map, &mut neighbors, y, x);
            }
            EW => {
                self.look_east_or_start(map, &mut neighbors, y, x);
                self.look_west_or_start(map, &mut neighbors, y, x);
            }
            NE => {
                self.look_south_or_start(map, &mut neighbors, y, x);
                self.look_west_or_start(map, &mut neighbors, y, x);
            }
            NW => {
                self.look_south_or_start(map, &mut neighbors, y, x);
                self.look_east_or_start(map, &mut neighbors, y, x);
            }
            SW => {
                self.look_north_or_start(map, &mut neighbors, y, x);
                self.look_east_or_start(map, &mut neighbors, y, x);
            }
            SE => {
                self.look_north_or_start(map, &mut neighbors, y, x);
                self.look_west_or_start(map, &mut neighbors, y, x);
            }
        }

        neighbors
    }
}

fn clean_map(
    start: Coordinate,
    loop_coords: &HashSet<Coordinate>,
    map: &[Vec<Tile>],
) -> Vec<Vec<Tile>> {
    let start_pipe = get_start_pipe(map, start);

    map.iter()
        .enumerate()
        .map(|(y, line)| {
            line.iter()
                .enumerate()
                .map(|(x, tile)| match tile {
                    Start => start_pipe,
                    pipe if loop_coords.contains(&Coordinate::new(y, x)) => *pipe,
                    _ => Ground,
                })
                .collect()
        })
        .collect()
}

fn get_start_pipe(map: &[Vec<Tile>], start: Coordinate) -> Tile {
    let neighbors = start.valid_neighbors(map);
    let north = neighbors.iter().any(|coord| coord.y < start.y);
    let south = neighbors.iter().any(|coord| coord.y > start.y);
    let west = neighbors.iter().any(|coord| coord.x < start.x);
    let east = neighbors.iter().any(|coord| coord.x > start.x);

    match (north, west, south, east) {
        (true, true, _, _) => NW,
        (true, _, true, _) => NS,
        (true, _, _, true) => NE,
        (_, true, true, _) => SW,
        (_, _, true, true) => SE,
        (_, true, _, true) => EW,
        _ => panic!("No valid tile to replace Start with was found"),
    }
}

fn part_1(input: &str) -> usize {
    let (map, start) = parse(input);
    let loop_coords = build_loop(start, &map);
    loop_coords.len() / 2
}

fn part_2(input: &str) -> usize {
    let (map, start) = parse(input);
    let loop_coords = build_loop(start, &map);
    let map = clean_map(start, &loop_coords, &map);
    let mut inside = false;

    map.into_iter()
        .flatten()
        .filter(|tile| match tile {
            Ground => inside,
            NS | NW | NE => {
                inside = !inside;
                false
            }
            _ => false,
        })
        .count()
}

fn main() {
    let input = include_str!("../input.txt");
    println!("part 1: {}, part 2: {}", part_1(input), part_2(input));
}

#[test]
fn example_1() {
    let example = include_str!("../example.txt");
    assert_eq!(8, part_1(example));
}

#[test]
fn example_2() {
    let example = include_str!("../example.txt");
    assert_eq!(1, part_2(example));
}
