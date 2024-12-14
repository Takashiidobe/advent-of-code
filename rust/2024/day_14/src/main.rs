use std::collections::HashSet;

use regex::Regex;

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
struct Coordinates {
    x: i64,
    y: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
struct Velocity {
    x: i64,
    y: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
struct Robot(Coordinates, Velocity);

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
struct Space {
    x: i64,
    y: i64,
}

pub trait ModuloSignedExt {
    fn modulo(&self, n: Self) -> Self;
}

macro_rules! modulo_signed_ext_impl {
    ($($t:ty)*) => ($(
        impl ModuloSignedExt for $t {
            #[inline]
            fn modulo(&self, n: Self) -> Self {
                (self % n + n) % n
            }
        }
    )*)
}

modulo_signed_ext_impl! { i64 }

impl Robot {
    // move the robot into its next square
    pub fn tick(&mut self, space: Space) {
        let Space { x: m, y: n } = space;
        let Velocity { x: dx, y: dy } = self.1;

        self.0.x = (self.0.x + dx).modulo(m);
        self.0.y = (self.0.y + dy).modulo(n);
    }
}

fn parse(input: &str) -> Vec<Robot> {
    let re = Regex::new(r"p=([0-9]+),([0-9]+) v=(\-?[0-9]+),(\-?[0-9]+)").unwrap();

    let mut res = vec![];
    for (_, [cx, cy, vx, vy]) in re.captures_iter(input).map(|c| c.extract()) {
        res.push(Robot(
            Coordinates {
                x: cx.parse().unwrap(),
                y: cy.parse().unwrap(),
            },
            Velocity {
                x: vx.parse().unwrap(),
                y: vy.parse().unwrap(),
            },
        ));
    }
    res
}

fn part_1(input: &str, height: i64, width: i64) -> u64 {
    let mut parsed = parse(input);

    for _ in 0..100 {
        for robot in parsed.iter_mut() {
            robot.tick(Space {
                x: width,
                y: height,
            });
        }
    }

    let mut corners = [0, 0, 0, 0];

    for robot in parsed {
        let Coordinates { x, y } = robot.0;
        if x == width / 2 || y == height / 2 {
            continue;
        }

        match (x < width / 2, y < height / 2) {
            (true, true) => corners[0] += 1,
            (true, false) => corners[1] += 1,
            (false, true) => corners[2] += 1,
            (false, false) => corners[3] += 1,
        }
    }

    corners.iter().product()
}

fn part_2(input: &str, height: i64, width: i64) -> u64 {
    let mut parsed = parse(input);
    let mut seconds = 1;

    loop {
        for robot in parsed.iter_mut() {
            robot.tick(Space {
                x: width,
                y: height,
            });
        }

        let unique_robots: HashSet<Coordinates> = HashSet::from_iter(parsed.iter().map(|x| x.0));

        if unique_robots.len() == parsed.len() {
            break;
        }
        seconds += 1
    }

    seconds
}

fn main() {
    println!("Part 1 Example: {}", part_1(EXAMPLE, 7, 11));
    println!("Part 1: {}", part_1(INPUT, 103, 101));
    println!("Part 2: {}", part_2(INPUT, 103, 101));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {
        assert_eq!(
            parse(EXAMPLE),
            vec![
                Robot(Coordinates { x: 0, y: 4 }, Velocity { x: 3, y: -3 }),
                Robot(Coordinates { x: 6, y: 3 }, Velocity { x: -1, y: -3 }),
                Robot(Coordinates { x: 10, y: 3 }, Velocity { x: -1, y: 2 }),
                Robot(Coordinates { x: 2, y: 0 }, Velocity { x: 2, y: -1 }),
                Robot(Coordinates { x: 0, y: 0 }, Velocity { x: 1, y: 3 }),
                Robot(Coordinates { x: 3, y: 0 }, Velocity { x: -2, y: -2 }),
                Robot(Coordinates { x: 7, y: 6 }, Velocity { x: -1, y: -3 }),
                Robot(Coordinates { x: 3, y: 0 }, Velocity { x: -1, y: -2 }),
                Robot(Coordinates { x: 9, y: 3 }, Velocity { x: 2, y: 3 }),
                Robot(Coordinates { x: 7, y: 3 }, Velocity { x: -1, y: 2 }),
                Robot(Coordinates { x: 2, y: 4 }, Velocity { x: 2, y: -3 }),
                Robot(Coordinates { x: 9, y: 5 }, Velocity { x: -3, y: -3 })
            ]
        );
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EXAMPLE, 7, 11), 12);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(EXAMPLE, 7, 11), 1);
    }
}
