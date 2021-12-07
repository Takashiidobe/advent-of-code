use std::cmp::{max, min};

fn get_input() -> &'static str {
    include_str!("../input.txt").trim()
}

fn parse_input(file: &str) -> Vec<usize> {
    let crabs = file.lines().next().expect("Didn't find any moves");

    let crabs: Vec<usize> = crabs
        .split(",")
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| x.parse().unwrap())
        .collect();

    let max_dist = crabs.iter().max().unwrap();

    let mut v = vec![0; *max_dist + 1];
    for crab in crabs {
        v[crab] += 1;
    }
    v
}

fn part_1(crabs: &mut Vec<usize>) -> usize {
    let mut min_fuel = usize::MAX;
    let crab_len = crabs.len();

    for i in 0..crab_len {
        let mut fuel_cost = 0;
        for (index, crab) in crabs.iter().enumerate() {
            if *crab > 0 {
                fuel_cost += (max(index, i) - min(index, i)) * crab;
            }
        }
        min_fuel = min(min_fuel, fuel_cost);
    }

    min_fuel
}

fn dist_to_fuel(dist: usize) -> usize {
    (1..=dist).fold(0, |acc, x| acc + x)
}

fn part_2(crabs: &mut Vec<usize>) -> usize {
    let mut min_fuel = usize::MAX;
    let crab_len = crabs.len();

    for i in 0..crab_len {
        let mut fuel_cost = 0;
        for (index, crab) in crabs.iter().enumerate() {
            if *crab > 0 {
                fuel_cost += (dist_to_fuel(max(index, i) - min(index, i))) * crab;
            }
        }
        min_fuel = min(min_fuel, fuel_cost);
    }

    min_fuel
}

fn main() {
    let input = parse_input(get_input());
    println!("part 1: {}", part_1(&mut input.clone()));
    println!("part 2: {}", part_2(&mut input.clone()));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let test_input = r#"16,1,2,0,4,2,7,1,2,14
"#;
        let input = parse_input(test_input);
        assert_eq!(part_1(&mut input.clone()), 37);
        assert_eq!(part_2(&mut input.clone()), 168);
    }

    #[test]
    fn test_2() {
        assert_eq!(dist_to_fuel(3), 6);
        assert_eq!(dist_to_fuel(11), 66);
    }
}
