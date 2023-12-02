use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq)]
enum Color {
    Blue(u32),
    Red(u32),
    Green(u32),
}

fn to_slice(input: &str) -> Vec<char> {
    input.chars().collect::<Vec<_>>()
}

// "1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
fn parse_color(input: &[char]) -> Option<(Color, usize)> {
    let (number, length) = parse_number(input);
    let input = &input[length..];

    let colors = &[" blue", " red", " green"];

    for (i, color) in colors.iter().enumerate() {
        if input.len() >= color.len() && input[..color.len()] == to_slice(color) {
            match i {
                0 => return Some((Color::Blue(number), length + color.len())),
                1 => return Some((Color::Red(number), length + color.len())),
                2 => return Some((Color::Green(number), length + color.len())),
                _ => unreachable!(),
            }
        }
    }

    None
}

fn parse_number(input: &[char]) -> (u32, usize) {
    let mut digits = vec![];
    let mut offset = 0;
    for c in input {
        if c.is_ascii_digit() {
            digits.push(c.to_digit(10).unwrap());
            offset += 1;
        } else {
            break;
        }
    }

    (digits.iter().fold(0, |acc, elem| acc * 10 + elem), offset)
}

fn validate(input: &HashMap<u32, Vec<Vec<Color>>>) -> HashMap<u32, bool> {
    let mut summed_colors = HashMap::new();
    for (game_id, games) in input {
        let mut result = true;
        for game in games {
            for color in game {
                match color {
                    Color::Blue(count) => {
                        if count > &14 {
                            result = false;
                        }
                    }
                    Color::Red(count) => {
                        if count > &12 {
                            result = false;
                        }
                    }
                    Color::Green(count) => {
                        if count > &13 {
                            result = false;
                        }
                    }
                }
            }
        }
        summed_colors.insert(*game_id, result);
    }
    summed_colors
}

fn parse(input: &str) -> HashMap<u32, Vec<Vec<Color>>> {
    let mut color_map: HashMap<u32, Vec<Vec<Color>>> = HashMap::new();
    for line in input.lines() {
        let replaced_line = line.replace("Game ", "");
        let mut slice_str = to_slice(&replaced_line);
        let (game_num, offset) = parse_number(&slice_str);
        slice_str = slice_str[offset + 2..].to_vec();
        let mut current_game = vec![];
        loop {
            // finish if its done
            if slice_str.is_empty() {
                color_map.entry(game_num).or_default().push(current_game);
                break;
            }
            // either parse a color
            if let Some((color, offset)) = parse_color(&slice_str) {
                current_game.push(color.clone());
                slice_str = slice_str[offset..].to_vec();
                continue;
            }
            // parse a comma or space
            if slice_str[0] == ',' || slice_str[0].is_ascii_whitespace() {
                slice_str = slice_str[1..].to_vec();
                continue;
            }
            // finish game if semicolon
            if slice_str[0] == ';' {
                slice_str = slice_str[1..].to_vec();
                color_map.entry(game_num).or_default().push(current_game);
                current_game = vec![];
            }
        }
    }
    color_map
}

fn sum_valid_colors(input: &HashMap<u32, bool>) -> u32 {
    let mut total = 0;
    for (game_id, valid) in input {
        if *valid {
            total += game_id;
        }
    }
    total
}

fn part_1(input: &str) -> u32 {
    let color_map = parse(input);
    let result = validate(&color_map);
    sum_valid_colors(&result)
}

fn part_2(input: &str) -> u32 {
    let color_map = parse(input);
    // flatten and take max of?
    let mut total = 0;
    for (_, games) in color_map {
        let mut max_red = 0;
        let mut max_blue = 0;
        let mut max_green = 0;
        for game in games {
            for color in game {
                match color {
                    Color::Red(count) => max_red = max_red.max(count),
                    Color::Blue(count) => max_blue = max_blue.max(count),
                    Color::Green(count) => max_green = max_green.max(count),
                }
            }
        }
        let power = max_red * max_blue * max_green;
        total += power;
    }
    total
}

fn main() {
    let input = include_str!("../input.txt");
    println!("part 1: {}, part 2: {}", part_1(input), part_2(input));
}

#[test]
fn example_pt_1() {
    let input = include_str!("../example.txt");
    let result = part_1(input);
    assert_eq!(result, 8);
}

#[test]
fn example_pt_2() {
    let input = include_str!("../example.txt");
    let result = part_2(input);
    assert_eq!(result, 2286);
}
