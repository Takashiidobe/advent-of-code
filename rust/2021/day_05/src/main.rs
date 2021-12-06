use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::{max, min};

lazy_static! {
    static ref RE: Regex = Regex::new(r"^([0-9]+),([0-9]+) -> ([0-9]+),([0-9]+)$").unwrap();
}

type Line = ((usize, usize), (usize, usize));
type Lines = Vec<Line>;

fn get_input() -> &'static str {
    include_str!("../input.txt")
}

fn parse_input(file: &str) -> Lines {
    let lines = file.trim().lines();

    let mut vec_lines = vec![];

    for line in lines {
        for cap in RE.captures_iter(line) {
            vec_lines.push((
                (cap[1].parse().unwrap(), cap[2].parse().unwrap()),
                (cap[3].parse().unwrap(), cap[4].parse().unwrap()),
            ));
        }
    }

    vec_lines
}

fn part_1(lines: &Lines) -> usize {
    let mut board = vec![vec![0; 1000]; 1000];
    for line in lines {
        let ((x1, y1), (x2, y2)) = line;

        if x1 == x2 {
            let min_y = min(y1, y2);
            let max_y = max(y1, y2);
            for y in *min_y..=*max_y {
                board[y][*x1] += 1;
            }
        }
        if y1 == y2 {
            let min_x = min(x1, x2);
            let max_x = max(x1, x2);
            for x in *min_x..=*max_x {
                board[*y1][x] += 1;
            }
        }
    }

    let mut count = 0;

    for i in 0..board.len() {
        count += board[i].iter().filter(|&&x| x > 1).count();
    }
    count
}

fn part_2(lines: &Lines) -> usize {
    let mut board = vec![vec![0; 1000]; 1000];
    for line in lines {
        let ((x1, y1), (x2, y2)) = line;
        let min_x = min(x1, x2);
        let max_x = max(x1, x2);
        let min_y = min(y1, y2);
        let max_y = max(y1, y2);

        if x1 == x2 {
            for y in *min_y..=*max_y {
                board[y][*x1] += 1;
            }
        }
        if y1 == y2 {
            for x in *min_x..=*max_x {
                board[*y1][x] += 1;
            }
        }
        if max_y - min_y == max_x - min_x {
            let dist = max_y - min_y;
            for slope in 0..=dist {
                board[min_y + slope][min_x + slope] += 1;
            }
        }
        // if max_y == max_x && min_y == min_x {
        //     for i in *min_x..=*max_x {
        //         for j in *min_y..=*max_y {
        //             board[j][i] += 1;
        //         }
        //     }
        // }
    }

    let mut count = 0;

    for i in 0..board.len() {
        count += board[i].iter().filter(|&&x| x > 1).count();
    }
    count
}

fn main() {
    let lines = parse_input(get_input());
    // println!("part 1: {}", part_1(&lines));
    println!("part 2: {}", part_2(&lines));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let test_input = r#"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2

"#;
        let lines = parse_input(test_input);
        assert_eq!(part_1(&lines), 5);
        assert_eq!(part_2(&lines), 12);
    }
}
