fn input() -> String {
    include_str!("../input.txt").to_string()
}

fn example() -> String {
    include_str!("../example.txt").to_string()
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Round {
    choice: Option<Move>,
    opponent: Option<Move>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum MatchResult {
    Win,
    Loss,
    Tie,
}

fn parse_input(input: String) -> Vec<Round> {
    let mut res = vec![];
    for line in input.lines() {
        let l = line.split_ascii_whitespace();
        let mut round = Round {
            choice: None,
            opponent: None,
        };
        for c in l {
            match c {
                "A" => round.opponent = Some(Move::Rock),
                "B" => round.opponent = Some(Move::Paper),
                "C" => round.opponent = Some(Move::Scissors),
                "X" => round.choice = Some(Move::Rock),
                "Y" => round.choice = Some(Move::Paper),
                "Z" => round.choice = Some(Move::Scissors),
                _ => {}
            }
        }
        res.push(round);
    }
    res
}

fn part_1(v: Vec<Round>) -> u32 {
    let mut res = 0;

    for round in v {
        res += calculate_points(&round);
    }

    res
}

use MatchResult::{Loss, Tie, Win};
use Move::{Paper, Rock, Scissors};

fn calculate_points(round: &Round) -> u32 {
    let round_result = calculate_result(round);
    let chosen_points = choice_points(round);
    let result_points = result_to_points(&round_result);

    chosen_points + result_points
}

fn result_to_points(outcome: &MatchResult) -> u32 {
    match outcome {
        Win => 6,
        Loss => 0,
        Tie => 3,
    }
}

fn choice_points(round: &Round) -> u32 {
    let Round { choice, opponent } = round;

    match choice.clone().unwrap() {
        Rock => 1,
        Paper => 2,
        Scissors => 3,
    }
}

fn calculate_result(round: &Round) -> MatchResult {
    let Round {
        ref choice,
        ref opponent,
    } = round;

    match (choice.clone().unwrap(), opponent.clone().unwrap()) {
        (Rock, Paper) | (Scissors, Paper) | (Rock, Scissors) => Win,
        (Rock, Rock) | (Paper, Paper) | (Scissors, Scissors) => Tie,
        (_, _) => Loss,
    }
}

fn part_2(v: Vec<Round>) -> u32 {
    // X means lose
    // Y means draw
    // Z means win
    for round in v {
        dbg!(round);
    }
    0
}

fn main() {
    let example = parse_input(example());
    let inp = parse_input(input());
    // println!("part 1: {}", part_1(inp));
    println!("part 2: {}", part_2(example));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let answer = part_1(parse_input(example()));

        assert_eq!(answer, 15);
    }

    #[test]
    fn test_2() {
        // let answer = part_2(parse_input(example()));

        // assert_eq!(answer, 45000);
    }
}
