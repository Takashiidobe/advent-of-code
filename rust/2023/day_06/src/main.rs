use regex::Regex;

fn parse_part_1(input: &str) -> Vec<(u64, u64)> {
    let re = Regex::new(r"\b(\d+)\b").unwrap();
    let mut times: Vec<u64> = vec![];
    let mut distances: Vec<u64> = vec![];
    for line in input.lines() {
        let split_line: Vec<_> = line.split(':').collect();
        if split_line[0] == "Time" {
            times = re
                .find_iter(split_line[1])
                .map(|p| p.as_str().parse().unwrap())
                .collect();
        }

        if split_line[0] == "Distance" {
            distances = re
                .find_iter(split_line[1])
                .map(|p| p.as_str().parse().unwrap())
                .collect();
        }
    }

    times.into_iter().zip(distances).collect()
}

fn part_1(input: &str) -> u64 {
    let parsed_input = parse_part_1(input);
    let mut ways_to_win: Vec<u64> = vec![];
    for (time, distance) in parsed_input {
        let mut curr_ways_to_win = 0;
        for held_time in 1..time - 1 {
            let distance_traveled = (time - held_time) * held_time;
            if distance_traveled > distance {
                curr_ways_to_win += 1;
            }
        }
        if curr_ways_to_win > 0 {
            ways_to_win.push(curr_ways_to_win);
        }
    }
    ways_to_win.into_iter().product()
}

fn parse_part_2(input: &str) -> (u64, u64) {
    let re = Regex::new(r"\b(\d+)\b").unwrap();
    let mut time_str = String::new();
    let mut distance_str = String::new();
    for line in input.lines() {
        let split_line: Vec<_> = line.split(':').collect();
        if split_line[0] == "Time" {
            let times: Vec<_> = re.find_iter(split_line[1]).map(|p| p.as_str()).collect();

            for time in times {
                time_str.push_str(time);
            }
        }

        if split_line[0] == "Distance" {
            let distances: Vec<_> = re.find_iter(split_line[1]).map(|p| p.as_str()).collect();

            for distance in distances {
                distance_str.push_str(distance);
            }
        }
    }

    (time_str.parse().unwrap(), distance_str.parse().unwrap())
}

fn part_2(input: &str) -> u64 {
    let (time, distance) = parse_part_2(input);

    let mut curr_ways_to_win = 0;
    for held_time in 1..time - 1 {
        let distance_traveled = (time - held_time) * held_time;
        if distance_traveled > distance {
            curr_ways_to_win += 1;
        }
    }

    curr_ways_to_win
}

fn main() {
    let input = include_str!("../input.txt");
    println!("part 1: {}, part 2: {}", part_1(input), part_2(input));
}

#[test]
fn example() {
    let example = include_str!("../example.txt");
    assert_eq!(288, part_1(example));
}

#[test]
fn example_2() {
    let example = include_str!("../example.txt");
    assert_eq!(71503, part_2(example));
}
