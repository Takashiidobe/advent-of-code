use std::collections::HashMap;

#[allow(dead_code)]
const EXAMPLE: &str = include_str!("example.txt");
#[allow(dead_code)]
const INPUT: &str = include_str!("input.txt");

fn parse(input: &str) -> Vec<(usize, usize)> {
    let mut points = vec![];
    for line in input.lines() {
        let split: Vec<_> = line.split(',').collect();
        let x: usize = split[0].parse().unwrap();
        let y: usize = split[1].parse().unwrap();
        points.push((x, y));
    }
    points
}

fn part_1(input: &str) -> usize {
    let points = parse(input);
    let mut max_so_far = 0;

    for (i, (x, y)) in points.iter().enumerate() {
        for (next_x, next_y) in &points[i..] {
            let curr = (next_y.max(y) - next_y.min(y) + 1) * (next_x.max(x) - next_x.min(x) + 1);
            max_so_far = max_so_far.max(curr);
        }
    }
    max_so_far
}

fn build_map(points: &[(usize, usize)]) -> HashMap<usize, (usize, usize)> {
    let mut spans: HashMap<usize, (usize, usize)> = HashMap::new();

    for &(x, y) in points {
        spans
            .entry(x)
            .and_modify(|(min_y, max_y)| {
                if y < *min_y {
                    *min_y = y;
                }
                if y > *max_y {
                    *max_y = y;
                }
            })
            .or_insert((y, y));
    }

    spans
}

fn parse2(input: &str) -> (Vec<(usize, usize)>, HashMap<usize, (usize, usize)>) {
    let mut reds = vec![];
    for line in input.lines() {
        let line = line.trim();
        let split: Vec<_> = line.split(',').collect();
        let x: usize = split[0].parse().unwrap();
        let y: usize = split[1].parse().unwrap();
        reds.push((x, y));
    }

    let mut greens = vec![];
    let (mut prev_x, mut prev_y) = *reds.last().unwrap();

    for &(x, y) in &reds {
        if x == prev_x {
            for i in (prev_y.min(y) + 1)..prev_y.max(y) {
                greens.push((x, i));
            }
        }
        if y == prev_y {
            for i in (prev_x.min(x) + 1)..prev_x.max(x) {
                greens.push((i, y));
            }
        }
        if !(x == prev_x || y == prev_y) {
            dbg!((prev_x, x), (prev_y, y));
            unreachable!()
        }

        prev_x = x;
        prev_y = y;
    }

    let mut combined = reds.clone();
    combined.extend_from_slice(&greens);

    let mapping = build_map(&combined);

    (reds, mapping)
}

fn part_2(input: &str) -> usize {
    let (reds, mapping) = parse2(input);

    let mut max_so_far = 0;

    for (i, &(x, y)) in reds.iter().enumerate() {
        'reds: for &(next_x, next_y) in &reds[i..] {
            let min_x = x.min(next_x);
            let max_x = x.max(next_x);
            let min_y = y.min(next_y);
            let max_y = y.max(next_y);

            for x_col in min_x..=max_x {
                let (span_min_y, span_max_y) = mapping[&x_col];
                if !(span_min_y <= min_y && span_max_y >= max_y) {
                    break 'reds;
                }
            }

            let curr = (max_y - min_y + 1) * (max_x - min_x + 1);
            max_so_far = max_so_far.max(curr);
        }
    }

    max_so_far
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
        assert_eq!(parse(EXAMPLE), vec![]);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EXAMPLE), 50);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(EXAMPLE), 0);
    }
}
