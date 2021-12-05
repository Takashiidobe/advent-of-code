use std::collections::HashSet;

type Moves = Vec<u8>;
type Boards = Vec<Board>;

#[derive(Default, Debug, Clone)]
struct Board {
    called: Vec<HashSet<u8>>,
    board: Vec<Vec<u8>>,
}

fn get_input() -> &'static str {
    include_str!("../input.txt")
}

fn parse_input(file: &str) -> (Moves, Boards) {
    let moves = file.lines().next().expect("Didn't find any moves");

    let moves: Moves = moves
        .split(",")
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut boards = vec![];
    let mut board = Board::default();

    for (index, line) in file.lines().skip(2).enumerate() {
        if (index + 1) % 6 == 0 {
            for _ in 0..5 {
                board.called.push(HashSet::default());
            }
            boards.push(board);
            board = Board::default();
        } else {
            board.board.push(
                line.split_whitespace()
                    .collect::<Vec<&str>>()
                    .iter()
                    .map(|x| x.parse::<u8>().unwrap())
                    .collect(),
            );
        }
    }

    (moves, boards)
}

fn is_winning(board: &Board) -> bool {
    let called = board.called.clone();

    for set in called.iter() {
        if set.len() == 5 {
            return true;
        }
    }

    for item in called[0].iter() {
        let mut is_in_all = true;
        for i in 1..5 {
            if called[i].contains(&item) == false {
                is_in_all = false;
            }
        }
        if is_in_all == true {
            return true;
        }
    }

    false
}

fn mark_board(m: u8, board: &mut Board) {
    for i in 0..5 {
        for j in 0..5 {
            if board.board[i][j] == m {
                board.called[i].insert(j as u8);
            }
        }
    }
}

fn calculate_score(board: &Board, m: u8) -> u32 {
    let mut unmarked = 0;
    for i in 0..5 {
        for j in 0..5 {
            if board.called[i].contains(&j) == false {
                unmarked += board.board[i][j as usize] as u32;
            }
        }
    }
    unmarked * (m as u32)
}

fn part_1(moves: Moves, boards: Boards) -> u32 {
    let mut boards = boards.clone();
    for m in moves {
        for mut board in boards.iter_mut() {
            mark_board(m, &mut board);
            if is_winning(&board) {
                return calculate_score(&board, m);
            }
        }
    }
    0
}

fn part_2(moves: Moves, boards: Boards) -> u32 {
    let mut boards = boards.clone();
    let mut winning_boards = HashSet::new();
    let mut winning_scores = vec![];
    let board_len = boards.len();
    for m in moves {
        for (index, mut board) in boards.iter_mut().enumerate() {
            if winning_boards.len() == board_len {
                return *winning_scores.last().unwrap();
            }
            if winning_boards.contains(&index) == true {
                continue;
            } else {
                mark_board(m, &mut board);
                if is_winning(&board) {
                    winning_boards.insert(index);
                    winning_scores.push(calculate_score(&board, m));
                }
            }
        }
    }
    0
}

fn main() {
    let (moves, boards) = parse_input(get_input());
    println!("part 1: {}", part_1(moves.clone(), boards.clone()));
    println!("part 2: {}", part_2(moves.clone(), boards.clone()));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let test_input = r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7

"#;
        let (moves, boards) = parse_input(test_input);
        assert_eq!(part_1(moves.clone(), boards.clone()), 4512);
        assert_eq!(part_2(moves.clone(), boards.clone()), 1924);
    }
}
