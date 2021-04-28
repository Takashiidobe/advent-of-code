use std::fs::read_to_string;

fn read_file(file_name: &str) -> String {
    read_to_string(file_name).unwrap()
}

fn main() {
    let input = read_file("../../input.txt");

    let input = input.lines().collect::<Vec<_>>();

    let mut pos = 0;

    for line in input {
        let symbol = line.as_bytes()[0] as char;

        let s: i32 = String::from_utf8_lossy(&line.as_bytes()[1..])
            .parse()
            .unwrap();

        match symbol {
            '+' => pos += s,
            '-' => pos -= s,
            _ => {}
        }
    }

    println!("{}", pos);
}
