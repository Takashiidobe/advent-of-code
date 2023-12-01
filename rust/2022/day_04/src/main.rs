use std::ops::RangeInclusive;

#[allow(unused)]
fn input() -> String {
    include_str!("../input.txt").to_string()
}

#[allow(unused)]
fn example() -> String {
    include_str!("../example.txt").to_string()
}

fn parse_input(input: String) -> Vec<(RangeInclusive<u32>, RangeInclusive<u32>)> {
    let mut res = vec![];
    for line in input.lines() {
        let split: Vec<_> = line.split(',').collect();
        let left: Vec<_> = split[0].split('-').collect();
        let right: Vec<_> = split[1].split('-').collect();
        let left_start: u32 = left[0].parse().unwrap();
        let left_end: u32 = left[1].parse().unwrap();
        let right_start: u32 = right[0].parse().unwrap();
        let right_end: u32 = right[1].parse().unwrap();
        res.push((
            RangeInclusive::new(left_start, left_end),
            RangeInclusive::new(right_start, right_end),
        ));
    }
    res
}

fn range_contains(left: RangeInclusive<u32>, right: RangeInclusive<u32>) -> bool {
    if left.start() <= right.start() && left.end() >= right.end() {
        true
    } else {
        right.start() <= left.start() && right.end() >= left.end()
    }
}

fn range_overlap(left: RangeInclusive<u32>, right: RangeInclusive<u32>) -> bool {
    if left.start() <= right.start() && left.end() >= right.start() {
        true
    } else {
        right.start() <= left.start() && right.end() >= left.start()
    }
}

fn part_1(v: Vec<(RangeInclusive<u32>, RangeInclusive<u32>)>) -> u32 {
    let mut ans = 0;
    for (left, right) in v {
        if range_contains(left, right) {
            ans += 1;
        }
    }
    ans
}

fn part_2(v: Vec<(RangeInclusive<u32>, RangeInclusive<u32>)>) -> u32 {
    let mut ans = 0;
    for (left, right) in v {
        if range_overlap(left, right) {
            ans += 1;
        }
    }
    ans
}

#[allow(unused)]
fn main() {
    let example = parse_input(example());
    let input = parse_input(input());
    println!("part 1: {}", part_1(input.clone()));
    println!("part 2: {}", part_2(input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let answer = part_1(parse_input(example()));

        assert_eq!(answer, 2);
    }

    #[test]
    fn test_2() {
        let answer = part_2(parse_input(example()));

        assert_eq!(answer, 4);
    }
}
