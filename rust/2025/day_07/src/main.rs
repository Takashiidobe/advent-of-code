use std::collections::{BTreeMap, BTreeSet};

#[allow(dead_code)]
const EXAMPLE: &str = include_str!("example.txt");
#[allow(dead_code)]
const INPUT: &str = include_str!("input.txt");

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tile {
    Splitter,
    Free,
}

fn parse(input: &str) -> (usize, Vec<Vec<Tile>>) {
    let mut lines: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    if let Some((beam_line, lines)) = lines.split_first_mut() {
        let beam_pos = beam_line.iter().position(|s| *s == 'S').unwrap();

        let tiles = lines
            .iter()
            .map(|l| {
                l.iter()
                    .map(|c| {
                        if *c == '^' {
                            Tile::Splitter
                        } else {
                            Tile::Free
                        }
                    })
                    .collect()
            })
            .collect();

        return (beam_pos, tiles);
    }

    unreachable!("failed to parse")
}

fn part_1(input: &str) -> i64 {
    let (beam_x, tiles) = parse(input);
    let mut beams = BTreeSet::from([beam_x]);
    let mut split_count = 0;
    let n = tiles[0].len();

    for tile in tiles {
        let mut new_tiles = BTreeSet::new();
        for beam in &beams {
            if tile[*beam] == Tile::Free {
                new_tiles.insert(*beam);
                continue;
            }
            let left_beam_pos = beam.checked_sub(1);
            let right_beam_pos = beam.checked_add(1);

            if let Some(left_beam) = left_beam_pos {
                new_tiles.insert(left_beam);
            }
            if let Some(right_beam) = right_beam_pos
                && right_beam < n
            {
                new_tiles.insert(right_beam);
            }
            split_count += 1;
        }
        beams = new_tiles;
    }

    split_count
}

fn part_2(input: &str) -> usize {
    // unique paths
    let (beam_x, tiles) = parse(input);
    let mut beams = BTreeMap::from([(beam_x, 1)]);
    let n = tiles[0].len();

    for tile in tiles {
        let mut new_tiles = BTreeMap::new();
        for (beam, count) in beams {
            if tile[beam] == Tile::Free {
                *new_tiles.entry(beam).or_default() += count;
                continue;
            }

            let left_beam_pos = beam.checked_sub(1);
            let right_beam_pos = beam.checked_add(1);

            if let Some(left_beam) = left_beam_pos {
                *new_tiles.entry(left_beam).or_default() += count;
            }
            if let Some(right_beam) = right_beam_pos
                && right_beam < n
            {
                *new_tiles.entry(right_beam).or_default() += count;
            }
        }
        beams = new_tiles;
    }

    beams.values().sum()
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
        use Tile::*;
        assert_eq!(
            parse(EXAMPLE),
            (
                7,
                vec![
                    vec![
                        Free, Free, Free, Free, Free, Free, Free, Free, Free, Free, Free, Free,
                        Free, Free, Free
                    ],
                    vec![
                        Free, Free, Free, Free, Free, Free, Free, Splitter, Free, Free, Free, Free,
                        Free, Free, Free
                    ],
                    vec![
                        Free, Free, Free, Free, Free, Free, Free, Free, Free, Free, Free, Free,
                        Free, Free, Free
                    ],
                    vec![
                        Free, Free, Free, Free, Free, Free, Splitter, Free, Splitter, Free, Free,
                        Free, Free, Free, Free
                    ],
                    vec![
                        Free, Free, Free, Free, Free, Free, Free, Free, Free, Free, Free, Free,
                        Free, Free, Free
                    ],
                    vec![
                        Free, Free, Free, Free, Free, Splitter, Free, Splitter, Free, Splitter,
                        Free, Free, Free, Free, Free
                    ],
                    vec![
                        Free, Free, Free, Free, Free, Free, Free, Free, Free, Free, Free, Free,
                        Free, Free, Free
                    ],
                    vec![
                        Free, Free, Free, Free, Splitter, Free, Splitter, Free, Free, Free,
                        Splitter, Free, Free, Free, Free
                    ],
                    vec![
                        Free, Free, Free, Free, Free, Free, Free, Free, Free, Free, Free, Free,
                        Free, Free, Free
                    ],
                    vec![
                        Free, Free, Free, Splitter, Free, Splitter, Free, Free, Free, Splitter,
                        Free, Splitter, Free, Free, Free
                    ],
                    vec![
                        Free, Free, Free, Free, Free, Free, Free, Free, Free, Free, Free, Free,
                        Free, Free, Free
                    ],
                    vec![
                        Free, Free, Splitter, Free, Free, Free, Splitter, Free, Free, Free, Free,
                        Free, Splitter, Free, Free
                    ],
                    vec![
                        Free, Free, Free, Free, Free, Free, Free, Free, Free, Free, Free, Free,
                        Free, Free, Free
                    ],
                    vec![
                        Free, Splitter, Free, Splitter, Free, Splitter, Free, Splitter, Free,
                        Splitter, Free, Free, Free, Splitter, Free
                    ],
                    vec![
                        Free, Free, Free, Free, Free, Free, Free, Free, Free, Free, Free, Free,
                        Free, Free, Free
                    ]
                ]
            )
        );
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EXAMPLE), 21);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(EXAMPLE), 40);
    }
}
