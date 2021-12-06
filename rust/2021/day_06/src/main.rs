use std::collections::VecDeque;

fn get_input() -> &'static str {
    include_str!("../input.txt").trim()
}

fn parse_input(file: &str) -> VecDeque<usize> {
    let fish = file.lines().next().expect("Didn't find any moves");

    let fish: Vec<usize> = fish
        .split(",")
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut slice = [0; 10];
    for f in fish {
        slice[f as usize] += 1;
    }

    VecDeque::from(slice)
}

fn part_1(fish: &mut VecDeque<usize>, days: u16) -> usize {
    for _ in 0..days {
        fish[9] += fish[0];
        fish[7] += fish[0];
        fish[0] = 0;
        fish.rotate_left(1);
    }

    fish.iter().fold(0, |acc, x| x + acc)
}

fn part_2(fish: &mut VecDeque<usize>, days: u16) -> usize {
    part_1(fish, days)
}

fn main() {
    let input = parse_input(get_input());
    println!("part 1: {}", part_1(&mut input.clone(), 80));
    println!("part 2: {}", part_2(&mut input.clone(), 256));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let test_input = r#"3,4,3,1,2
"#;
        let input = parse_input(test_input);
        assert_eq!(part_1(&mut input.clone(), 80), 5934);
        assert_eq!(part_1(&mut input.clone(), 256), 26984457539);
    }
}
