use std::collections::HashSet;

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn get_input(s: &str) -> Vec<Vec<bool>> {
    let s = s.trim();
    let mut v = vec![];
    for line in s.lines() {
        let mut line_vec = vec![];
        for l in line.chars() {
            match l {
                '1' => line_vec.push(true),
                _ => line_vec.push(false),
            }
        }
        v.push(line_vec);
    }
    v
}

fn part_1(instructions: &Vec<Vec<bool>>) -> u32 {
    let instructions = transpose(instructions.clone());

    let mut gamma_str = String::default();
    let mut epsilon_str = String::default();

    for instruction_list in instructions {
        let true_count = instruction_list.iter().filter(|&&x| x == true).count();
        let false_count = instruction_list.iter().filter(|&&x| x == false).count();
        if true_count > false_count {
            gamma_str.push('1');
            epsilon_str.push('0');
        } else {
            gamma_str.push('0');
            epsilon_str.push('1');
        }
    }
    let gamma = u32::from_str_radix(&gamma_str, 2).unwrap();
    let epsilon = u32::from_str_radix(&epsilon_str, 2).unwrap();
    gamma * epsilon
}

fn part_2(instructions: &Vec<Vec<bool>>) -> u32 {
    let mut co2_scrubber_rating_str = String::default();
    let mut oxygen_generator_str = String::default();

    let mut set = HashSet::new();

    for i in 0..instructions[0].len() {
        set.insert(i);
    }

    let mut pos = 0;

    while set.len() > 1 {
        let mut true_set = HashSet::new();
        let mut false_set = HashSet::new();
        for i in set.iter() {
            if instructions[*i][pos] == true {
                true_set.insert(*i);
            } else {
                false_set.insert(*i);
            }
        }
        if true_set.len() >= false_set.len() {
            set = true_set;
        } else {
            set = false_set;
        }

        pos += 1;
    }

    for i in set.iter() {
        let instruction_str: Vec<&str> = instructions[*i]
            .iter()
            .map(|&x| if x == true { "1" } else { "0" })
            .collect();
        oxygen_generator_str = instruction_str.join("");
    }

    let mut set = HashSet::new();

    for i in 0..instructions[0].len() {
        set.insert(i);
    }

    let mut pos = 0;

    while set.len() > 1 {
        let mut true_set = HashSet::new();
        let mut false_set = HashSet::new();
        for i in set.iter() {
            if instructions[*i][pos] == true {
                true_set.insert(*i);
            } else {
                false_set.insert(*i);
            }
        }
        if false_set.len() >= true_set.len() {
            set = false_set;
        } else {
            set = true_set;
        }

        pos += 1;
    }

    for i in set.iter() {
        let instruction_str: Vec<&str> = instructions[*i]
            .iter()
            .map(|&x| if x == true { "1" } else { "0" })
            .collect();
        co2_scrubber_rating_str = instruction_str.join("");
    }

    let oxygen_generator = u32::from_str_radix(&oxygen_generator_str, 2).unwrap();
    let co2_scrubber_rating = u32::from_str_radix(&co2_scrubber_rating_str, 2).unwrap();

    println!(
        "oxygen (should be 23): {}, co2 (should be 10) : {}",
        oxygen_generator, co2_scrubber_rating
    );
    // println!(
    //     "oxygen (should be 1071): {}, co2 (should be 3706) : {}",
    //     oxygen_generator, co2_scrubber_rating
    // );

    co2_scrubber_rating * oxygen_generator
}

fn main() {
    let s = include_str!("../test_input.txt");
    let v = get_input(&s);
    println!("part 1: {}", part_1(&v));
    println!("part 2: {}", part_2(&v));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let s = r#"
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
        "#
        .trim();
        assert_eq!(part_1(&get_input(&s)), 198);
    }
}
