use regex::Regex;

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

#[derive(Debug, Clone, Copy, PartialEq)]
struct Button {
    x: i64,
    y: i64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Prize {
    x: i64,
    y: i64,
}

fn parse(input: &str) -> Vec<(Button, Button, Prize)> {
    let re = Regex::new(r"Button A: X\+([0-9]+), Y\+([0-9]+)\nButton B: X\+([0-9]+), Y\+([0-9]+)\nPrize: X=([0-9]+), Y=([0-9]+)").unwrap();

    let mut res = vec![];
    for (_, [ax, ay, bx, by, px, py]) in re.captures_iter(input).map(|c| c.extract()) {
        res.push((
            Button {
                x: ax.parse().unwrap(),
                y: ay.parse().unwrap(),
            },
            Button {
                x: bx.parse().unwrap(),
                y: by.parse().unwrap(),
            },
            Prize {
                x: px.parse().unwrap(),
                y: py.parse().unwrap(),
            },
        ));
    }
    res
}

fn part_1(input: &str) -> i64 {
    let parsed = parse(input);
    let mut total = 0;

    for (Button { x: ax, y: ay }, Button { x: bx, y: by }, Prize { x: px, y: py }) in parsed {
        let mut min_cost: Option<i64> = None;
        for i in 0..=100 {
            for j in 0..=100 {
                if (ax * i) + (bx * j) == px && (ay * i) + (by * j) == py {
                    if let Some(min_so_far) = min_cost {
                        min_cost = Some(min_so_far.min(i * 3 + j));
                    } else {
                        min_cost = Some(i * 3 + j);
                    }
                }
            }
        }
        if let Some(cost) = min_cost {
            total += cost;
        }
    }
    total
}

fn part_2(input: &str) -> i64 {
    let parsed = parse(input);
    let mut total = 0;

    for (Button { x: ax, y: ay }, Button { x: bx, y: by }, Prize { x: px, y: py }) in parsed {
        let (px, py) = (px + 10000000000000, py + 10000000000000);
        let det = ax * by - ay * bx;
        let a = (px * by - py * bx) / det;
        let b = (py * ax - px * ay) / det;
        total += if (ax * a + bx * b, ay * a + by * b) == (px, py) {
            a * 3 + b
        } else {
            0
        };
    }
    total
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
                (
                    Button { x: 94, y: 34 },
                    Button { x: 22, y: 67 },
                    Prize { x: 8400, y: 5400 }
                ),
                (
                    Button { x: 26, y: 66 },
                    Button { x: 67, y: 21 },
                    Prize { x: 12748, y: 12176 }
                ),
                (
                    Button { x: 17, y: 86 },
                    Button { x: 84, y: 37 },
                    Prize { x: 7870, y: 6450 }
                ),
                (
                    Button { x: 69, y: 23 },
                    Button { x: 27, y: 71 },
                    Prize { x: 18641, y: 10279 }
                )
            ]
        );
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EXAMPLE), 480);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(EXAMPLE), 875318608908);
    }
}
