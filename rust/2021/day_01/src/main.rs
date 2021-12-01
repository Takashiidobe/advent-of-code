use std::fs::read_to_string;

fn get_input() -> String {
    let s = read_to_string("../input.txt").unwrap();
    s.trim().to_string()
}

fn input_to_vector(s: &str) -> Vec<usize> {
    s.split('\n')
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| x.parse::<usize>().unwrap())
        .collect()
}

fn part_1() {
    let s = get_input();
    let v = input_to_vector(&s);
    let zipped = v.iter().zip(v.iter().skip(1));
    let mut count = 0;
    for (i, j) in zipped {
        if j > i {
            count += 1;
        }
    }
    println!("{}", count);
}

fn part_2() {
    let s = get_input();
    let v = input_to_vector(&s);
    let zipped = (0..v.len()).skip(2).zip((0..v.len()).skip(3));
    let mut count = 0;
    for (i, j) in zipped {
        let first = v[i] + v[i - 1] + v[i - 2];
        let second = v[j] + v[j - 1] + v[j - 2];
        if second > first {
            count += 1;
        }
    }
    println!("{}", count);
}

fn main() {
    part_2();
}
