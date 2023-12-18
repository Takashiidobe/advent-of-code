#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Instruction {
    dir: Dir,
    amount: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Coordinate {
    x: i64,
    y: i64,
}

fn calc_area(instructions: &[Instruction]) -> i64 {
    let (area, perimeter, _) = instructions.iter().fold(
        (0, 0, Coordinate { x: 0, y: 0 }),
        |(area, perimeter, pos), Instruction { dir, amount }| {
            let new_pos = match dir {
                Dir::Up => Coordinate {
                    x: pos.x + amount,
                    y: pos.y,
                },
                Dir::Down => Coordinate {
                    x: pos.x - amount,
                    y: pos.y,
                },
                Dir::Left => Coordinate {
                    x: pos.x,
                    y: pos.y - amount,
                },
                Dir::Right => Coordinate {
                    x: pos.x,
                    y: pos.y + amount,
                },
            };
            let new_area = area + (pos.x * new_pos.y - new_pos.x * pos.y);
            let new_perimeter = (new_pos.x - pos.x).abs() + (new_pos.y - pos.y).abs() + perimeter;
            (new_area, new_perimeter, new_pos)
        },
    );

    (area.abs() + perimeter) / 2 + 1
}

fn parse_part_1(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let (instr, _) = line.split_once(" (").unwrap();
            let (dir, amount) = instr.split_once(' ').unwrap();
            let dir = match dir {
                "U" => Dir::Up,
                "D" => Dir::Down,
                "L" => Dir::Left,
                "R" => Dir::Right,
                _ => unreachable!(),
            };
            let amount = amount.parse().unwrap();
            Instruction { dir, amount }
        })
        .collect()
}

fn parse_part_2(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let line = line.strip_suffix(')').unwrap();
            let (_, hex) = line.split_once("(#").unwrap();
            let (amount, dir) = hex.split_at(5);
            let amount = i64::from_str_radix(amount, 16).unwrap();

            let dir = match dir {
                "3" => Dir::Up,
                "1" => Dir::Down,
                "2" => Dir::Left,
                "0" => Dir::Right,
                _ => unreachable!(),
            };

            Instruction { dir, amount }
        })
        .collect()
}

fn part_1(input: &str) -> i64 {
    calc_area(&parse_part_1(input))
}

fn part_2(input: &str) -> i64 {
    calc_area(&parse_part_2(input))
}

fn main() {
    let input = include_str!("../input.txt");
    println!("part 1: {}, part 2: {}", part_1(input), part_2(input));
}

#[test]
fn example_1() {
    let example = include_str!("../example.txt");
    assert_eq!(62, part_1(example));
}

#[test]
fn example_2() {
    let example = include_str!("../example.txt");
    assert_eq!(952408144115, part_2(example));
}
