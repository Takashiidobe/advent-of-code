fn calibrate(input: &str) -> u32 {
    let mut total = 0;
    for line in input.lines() {
        let digits: Vec<_> = line
            .chars()
            .filter(|s| s.is_ascii_digit())
            .map(|d| d.to_digit(10).unwrap())
            .collect();
        let first_digit = digits.first().unwrap();
        let last_digit = digits.last().unwrap();
        let number: u32 = format!("{}{}", first_digit, last_digit).parse().unwrap();
        total += number;
    }

    total
}

fn to_slice(input: &str) -> Vec<char> {
    input.chars().collect::<Vec<_>>()
}

fn spelled_digit(input: &[char]) -> Option<u32> {
    let spelled_digits = &[
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    for (i, digit) in spelled_digits.iter().enumerate() {
        if input.len() >= digit.len() && input[0..digit.len()] == to_slice(digit) {
            return Some(i as u32 + 1);
        }
    }

    None
}

fn calibrate_part_2(input: &str) -> u32 {
    let mut total = 0;
    for line in input.lines() {
        let mut digits = vec![];
        let characters: Vec<_> = line.chars().collect();
        for (i, c) in characters.iter().enumerate() {
            if c.is_ascii_digit() {
                digits.push(c.to_digit(10).unwrap());
            } else if let Some(num) = spelled_digit(&characters[i..]) {
                digits.push(num);
            }
        }

        let first_digit = digits.first().unwrap();
        let last_digit = digits.last().unwrap();
        let number: u32 = format!("{}{}", first_digit, last_digit).parse().unwrap();
        total += number;
    }

    total
}

fn main() {
    let input = include_str!("../input.txt");
    let part_1 = calibrate(input);
    let part_2 = calibrate_part_2(input);
    println!("part 1: {}, part 2: {}", part_1, part_2);
}

#[test]
fn part_1() {
    let input = include_str!("../example.txt");
    let output = calibrate(input);
    assert_eq!(output, 142);
}

#[test]
fn part_2() {
    let input = include_str!("../example_2.txt");
    let output = calibrate_part_2(input);
    assert_eq!(output, 281);
}
