use std::cmp::max;
use std::fs::read_to_string;

fn read_input(file_name: &str) -> Vec<String> {
    read_to_string(file_name)
        .unwrap()
        .split("\n")
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
}

enum Command {
    Toggle,
    TurnOn,
    TurnOff,
}

fn main() {
    let input = read_input("../input.txt");

    let mut grid: Vec<Vec<i32>> = Vec::new();

    for i in 0..1000 {
        grid.push(Vec::new());
        for j in 0..1000 {
            grid[i].push(0);
        }
    }

    fn toggle_lights(grid: &mut Vec<Vec<i32>>, command: Command, pos: Vec<usize>) {
        match command {
            Command::Toggle => {
                for i in pos[0]..=pos[2] {
                    for j in pos[1]..=pos[3] {
                        grid[i][j] += 2;
                    }
                }
            }
            Command::TurnOn => {
                for i in pos[0]..=pos[2] {
                    for j in pos[1]..=pos[3] {
                        grid[i][j] += 1;
                    }
                }
            }
            Command::TurnOff => {
                for i in pos[0]..=pos[2] {
                    for j in pos[1]..=pos[3] {
                        grid[i][j] -= 1;
                        if grid[i][j] < 0 {
                            grid[i][j] = 0;
                        }
                    }
                }
            }
        }
    }

    for line in input {
        if line.starts_with("turn on ") {
            let trimmed_line = line.trim_start_matches("turn on ");
            let parsed_line: Vec<&str> = trimmed_line.split(" through ").collect();
            let mut trimmed_line = Vec::new();
            for pair in parsed_line {
                let pair: Vec<&str> = pair.split(",").collect();
                trimmed_line.push(pair[0].parse::<usize>().unwrap());
                trimmed_line.push(pair[1].parse::<usize>().unwrap());
            }
            toggle_lights(&mut grid, Command::TurnOn, trimmed_line);
        } else if line.starts_with("turn off ") {
            let trimmed_line = line.trim_start_matches("turn off ");
            let parsed_line: Vec<&str> = trimmed_line.split(" through ").collect();
            let mut trimmed_line = Vec::new();
            for pair in parsed_line {
                let pair: Vec<&str> = pair.split(",").collect();
                trimmed_line.push(pair[0].parse::<usize>().unwrap());
                trimmed_line.push(pair[1].parse::<usize>().unwrap());
            }
            toggle_lights(&mut grid, Command::TurnOff, trimmed_line);
        } else if line.starts_with("toggle ") {
            let trimmed_line = line.trim_start_matches("toggle ");
            let parsed_line: Vec<&str> = trimmed_line.split(" through ").collect();
            let mut trimmed_line = Vec::new();
            for pair in parsed_line {
                let pair: Vec<&str> = pair.split(",").collect();
                trimmed_line.push(pair[0].parse::<usize>().unwrap());
                trimmed_line.push(pair[1].parse::<usize>().unwrap());
            }
            toggle_lights(&mut grid, Command::Toggle, trimmed_line);
        }
    }

    let mut count = 0;

    for i in 0..1000 {
        for j in 0..1000 {
            count += grid[i][j];
        }
    }
    assert_eq!(17836115, count);
    println!("{}", count);
}
