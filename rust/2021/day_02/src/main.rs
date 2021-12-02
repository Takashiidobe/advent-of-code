#[derive(Debug)]
enum Directions {
    Forward(u32),
    Down(u32),
    Up(u32),
}

use Directions::*;

fn get_input() -> Vec<Directions> {
    let s = include_str!("../input.txt").trim();
    let mut v = vec![];
    for line in s.lines() {
        let split_line: Vec<&str> = line.split_whitespace().collect();
        match split_line[0] {
            "forward" => v.push(Forward(split_line[1].parse().unwrap())),
            "down" => v.push(Down(split_line[1].parse().unwrap())),
            "up" => v.push(Up(split_line[1].parse().unwrap())),
            _ => (),
        }
    }
    v
}

fn part_1(directions: &Vec<Directions>) -> u32 {
    let mut x = 0;
    let mut y = 0;

    for direction in directions {
        match direction {
            Forward(n) => x += n,
            Down(n) => y += n,
            Up(n) => y -= n,
        }
    }
    x * y
}

fn part_2(directions: &Vec<Directions>) -> u32 {
    let mut x = 0;
    let mut y = 0;
    let mut z = 0;

    for direction in directions {
        match direction {
            Forward(n) => {
                x += n;
                y += z * n;
            }
            Down(n) => {
                z += n;
            }
            Up(n) => {
                z -= n;
            }
        }
    }
    x * y
}

fn main() {
    let v = get_input();
    println!("part 1: {}", part_1(&v));
    println!("part 2: {}", part_2(&v));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let v = vec![Forward(5), Down(5), Forward(8), Up(3), Down(8), Forward(2)];
        assert_eq!(part_1(&v), 150);
        assert_eq!(part_2(&v), 900);
    }

    #[test]
    fn test_2() {
        let v = vec![
            Forward(5),
            Down(5),
            Forward(8),
            Up(3),
            Down(8),
            Forward(2),
            Down(3),
        ];
        assert_eq!(part_1(&v), 195);
        assert_eq!(part_2(&v), 900);
    }
}
