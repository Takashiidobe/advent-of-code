use std::fs::read_to_string;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r"^(N|S|E|W|L|R|F|)([0-9]+)$").unwrap();
}

#[derive(Debug, Clone)]
enum Navigation {
    North,
    East,
    South,
    West,
    Left,
    Right,
    Forward,
}

fn parse() -> Vec<(Navigation, i32)> {
    let input = read_to_string("../input.txt").unwrap();
    let mut output = vec![];

    for line in input.lines() {
        let captures = RE.captures(line).unwrap();
        match &captures[1] {
            "N" => output.push((Navigation::North, captures[2].parse::<i32>().unwrap())),
            "E" => output.push((Navigation::East, captures[2].parse::<i32>().unwrap())),
            "S" => output.push((Navigation::South, captures[2].parse::<i32>().unwrap())),
            "W" => output.push((Navigation::West, captures[2].parse::<i32>().unwrap())),
            "L" => output.push((Navigation::Left, captures[2].parse::<i32>().unwrap())),
            "R" => output.push((Navigation::Right, captures[2].parse::<i32>().unwrap())),
            "F" => output.push((Navigation::Forward, captures[2].parse::<i32>().unwrap())),
            _ => {}
        }
    }
    output
}

fn part_one() -> i32 {
    let mut coords = (0, 0);
    let mut direction: i32 = 90;
    let input = parse();

    for (dir, val) in input {
        match dir {
            Navigation::North => coords.1 += val,
            Navigation::East => coords.0 += val,
            Navigation::South => coords.1 -= val,
            Navigation::West => coords.0 -= val,
            Navigation::Right => direction = (direction + val) % 360,
            Navigation::Left => direction = (360 + direction - val) % 360,
            Navigation::Forward => match direction {
                0 => coords.1 += val,
                90 => coords.0 += val,
                180 => coords.1 -= val,
                270 => coords.0 -= val,
                _ => {}
            },
        }
    }

    coords.1.abs() + coords.0.abs()
}

fn part_two() -> i32 {
    let mut coords = (0, 0);
    let mut waypoint = (10, 1);
    let input = parse();

    for (dir, val) in input {
        match dir {
            Navigation::North => waypoint.1 += val,
            Navigation::East => waypoint.0 += val,
            Navigation::South => waypoint.1 -= val,
            Navigation::West => waypoint.0 -= val,
            Navigation::Right => {
                let direction = val % 360;
                match direction {
                    90 => waypoint = (waypoint.1, -waypoint.0),
                    180 => waypoint = (-waypoint.0, -waypoint.1),
                    270 => waypoint = (-waypoint.1, waypoint.0),
                    _ => {}
                }
            }
            Navigation::Left => {
                let direction = val % 360;
                match direction {
                    90 => waypoint = (-waypoint.1, waypoint.0),
                    180 => waypoint = (-waypoint.0, -waypoint.1),
                    270 => waypoint = (waypoint.1, -waypoint.0),
                    _ => {}
                }
            }
            Navigation::Forward => {
                coords.0 += waypoint.0 * val;
                coords.1 += waypoint.1 * val;
            }
        }
    }

    coords.1.abs() + coords.0.abs()
}

fn main() {
    println!("part one: {}", part_one());
    println!("part two: {}", part_two());
}
