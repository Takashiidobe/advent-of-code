use std::collections::HashSet;

#[derive(Hash, Debug, Clone, PartialOrd, Ord, PartialEq, Eq)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            'T' => Self::Ten,
            'J' => Self::Jack,
            'Q' => Self::Queen,
            'K' => Self::King,
            'A' => Self::Ace,
            _ => unreachable!(),
        }
    }
}

#[derive(Hash, Debug, Clone, PartialOrd, Ord, PartialEq, Eq)]
enum Card2 {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

impl From<char> for Card2 {
    fn from(value: char) -> Self {
        match value {
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            'T' => Self::Ten,
            'J' => Self::Joker,
            'Q' => Self::Queen,
            'K' => Self::King,
            'A' => Self::Ace,
            _ => unreachable!(),
        }
    }
}

#[derive(Hash, Debug, Clone, PartialOrd, Ord, PartialEq, Eq)]
enum Hand<C> {
    HighCard((C, C, C, C, C)),
    OnePair((C, C, C, C, C)),
    TwoPair((C, C, C, C, C)),
    ThreeOfAKind((C, C, C, C, C)),
    FullHouse((C, C, C, C, C)),
    FourOfAKind((C, C, C, C, C)),
    FiveOfAKind((C, C, C, C, C)),
}

impl From<Vec<Card>> for Hand<Card> {
    fn from(value: Vec<Card>) -> Self {
        let mut value = value.clone();
        let card_tuple = (
            value[0].clone(),
            value[1].clone(),
            value[2].clone(),
            value[3].clone(),
            value[4].clone(),
        );
        value.sort();
        let card_set: HashSet<Card> = HashSet::from_iter(value.clone());

        if card_set.len() == 5 {
            Self::HighCard(card_tuple)
        } else if card_set.len() == 4 {
            Self::OnePair(card_tuple)
        } else if card_set.len() == 3 {
            if value[0] == value[2] || value[1] == value[3] || value[2] == value[4] {
                Self::ThreeOfAKind(card_tuple)
            } else {
                Self::TwoPair(card_tuple)
            }
        } else if card_set.len() == 2 {
            if value[0] == value[3] || value[1] == value[4] {
                Self::FourOfAKind(card_tuple)
            } else {
                Self::FullHouse(card_tuple)
            }
        } else if card_set.len() == 1 {
            Self::FiveOfAKind(card_tuple)
        } else {
            unreachable!()
        }
    }
}

use counter::Counter;

impl From<Vec<Card2>> for Hand<Card2> {
    fn from(value: Vec<Card2>) -> Self {
        let mut value = value.clone();
        let card_tuple = (
            value[0].clone(),
            value[1].clone(),
            value[2].clone(),
            value[3].clone(),
            value[4].clone(),
        );
        value.sort();
        let mut counter: Counter<_> = Counter::from_iter(value.clone());
        if counter[&Card2::Joker] > 0 {
            let joker_count = counter[&Card2::Joker];
            let mut most_common = counter.most_common();
            most_common.sort_by(|a, b| a.1.cmp(&b.1).then(b.0.cmp(&a.0)));
            let most_common_ordered = most_common.first();

            if let Some((item, zero)) = most_common_ordered {
                counter.remove(&Card2::Joker);
                counter[item] += joker_count;
            }
        }
        value = vec![];
        let mut most_common = counter.most_common();
        most_common.sort_by(|a, b| a.1.cmp(&b.1).then(b.0.cmp(&a.0)));
        most_common.reverse();
        for (item, frequency) in most_common {
            for _ in 0..frequency {
                value.push(item.clone());
            }
        }

        if counter.len() == 5 {
            Self::HighCard(card_tuple)
        } else if counter.len() == 4 {
            Self::OnePair(card_tuple)
        } else if counter.len() == 3 {
            if value[0] == value[2] || value[1] == value[3] || value[2] == value[4] {
                Self::ThreeOfAKind(card_tuple)
            } else {
                Self::TwoPair(card_tuple)
            }
        } else if counter.len() == 2 {
            if value[0] == value[3] || value[1] == value[4] {
                Self::FourOfAKind(card_tuple)
            } else {
                Self::FullHouse(card_tuple)
            }
        } else if counter.len() == 1 {
            Self::FiveOfAKind(card_tuple)
        } else {
            unreachable!()
        }
    }
}

fn parse(input: &str) -> Vec<(Hand<Card>, usize)> {
    let mut result = vec![];
    for line in input.lines() {
        let split: Vec<_> = line.split_ascii_whitespace().collect();
        let (str_hand, rank) = (split[0], split[1].parse::<usize>().unwrap());

        let hand: Vec<_> = str_hand.chars().map(Card::from).collect();

        result.push((Hand::from(hand), rank));
    }

    result.sort();

    result
}

fn parse_part_2(input: &str) -> Vec<(Hand<Card2>, usize)> {
    let mut result: Vec<(Hand<Card2>, usize)> = vec![];
    for line in input.lines() {
        let split: Vec<_> = line.split_ascii_whitespace().collect();
        let (str_hand, rank) = (split[0], split[1].parse::<usize>().unwrap());

        let hand: Vec<_> = str_hand.chars().map(Card2::from).collect();

        result.push((Hand::from(hand), rank));
    }

    result.sort();

    result
}

fn part_1(input: &str) -> usize {
    let hands = parse(input);

    let mut winnings = 0;

    for (i, (hand, bid)) in hands.into_iter().enumerate() {
        let rank = i + 1;
        winnings += bid * rank;
    }

    winnings
}

fn part_2(input: &str) -> usize {
    let hands = parse_part_2(input);

    let mut winnings = 0;

    for (i, (hand, bid)) in hands.into_iter().enumerate() {
        let rank = i + 1;
        // dbg!(hand, rank, bid);
        winnings += bid * rank;
    }

    winnings
}

fn main() {
    let input = include_str!("../input.txt");
    let example = include_str!("../example.txt");
    println!("{}", part_2(input));
    // println!("part 1: {}, part 2: {}", part_1(input), part_2(input));
}

#[test]
fn example() {
    let example = include_str!("../example.txt");
    assert_eq!(6440, part_1(example));
}

#[test]
fn example_2() {
    let example = include_str!("../example.txt");
    assert_eq!(5905, part_2(example));
}
