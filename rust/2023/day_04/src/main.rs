use std::collections::HashSet;

use regex::Regex;

fn part_1(input: &str) -> u32 {
    let mut points = 0;
    let re = Regex::new(r"\b(\d+)\b").unwrap();
    for line in input.lines() {
        let split: Vec<_> = line.split(": ").collect();
        let tickets: Vec<_> = split[1].split(" | ").collect();
        let winning_tickets = tickets[0];
        let tickets_bought = tickets[1];

        let winning_ticket_nums: HashSet<i32> = re
            .find_iter(winning_tickets)
            .map(|m| m.as_str().parse().unwrap())
            .collect();

        let bought_ticket_nums: Vec<i32> = re
            .find_iter(tickets_bought)
            .map(|m| m.as_str().parse().unwrap())
            .collect();

        let mut curr_power = 0;

        for bought_ticket in bought_ticket_nums {
            if winning_ticket_nums.contains(&bought_ticket) {
                curr_power += 1;
            }
        }
        if curr_power > 0 {
            points += 2u32.pow(curr_power - 1);
        }
    }
    points
}

// we win instances of cards
fn part_2(input: &str) -> u32 {
    let mut copies = vec![0; 300];
    let re = Regex::new(r"\b(\d+)\b").unwrap();
    for (index, line) in input.lines().enumerate() {
        let card_num = index + 1;
        let split: Vec<_> = line.split(": ").collect();
        let tickets: Vec<_> = split[1].split(" | ").collect();
        let winning_tickets = tickets[0];
        let tickets_bought = tickets[1];

        let winning_ticket_nums: HashSet<i32> = re
            .find_iter(winning_tickets)
            .map(|m| m.as_str().parse().unwrap())
            .collect();

        let bought_ticket_nums: Vec<i32> = re
            .find_iter(tickets_bought)
            .map(|m| m.as_str().parse().unwrap())
            .collect();

        let mut cards_won = 0;

        for bought_ticket in bought_ticket_nums {
            if winning_ticket_nums.contains(&bought_ticket) {
                cards_won += 1;
            }
        }
        copies[card_num] += 1;
        for i in card_num + 1..card_num + cards_won + 1 {
            copies[i] += copies[card_num];
        }
    }
    copies.into_iter().sum()
}

fn main() {
    let input = include_str!("../input.txt");
    let part_1 = part_1(input);
    let part_2 = part_2(input);
    println!("part 1: {}, part 2: {}", part_1, part_2);
}

#[test]
fn example() {
    let input = include_str!("../example.txt");
    let result = part_1(input);
    assert_eq!(result, 13);
}

#[test]
fn example_2() {
    let input = include_str!("../example.txt");
    let result = part_2(input);
    assert_eq!(result, 30);
}
