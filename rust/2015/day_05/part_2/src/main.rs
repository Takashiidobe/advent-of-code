use std::fs::read_to_string;

fn read_input(file_name: &str) -> String {
    read_to_string(file_name).unwrap()
}

fn letter_between(s: &String) -> bool {
    let mut i = 0;
    let mut j = 1;
    let mut k = 2;

    while k < s.len() {
        let first = s.chars().nth(i).unwrap();
        let second = s.chars().nth(j).unwrap();
        let third = s.chars().nth(k).unwrap();

        if first == third {
            return true;
        }

        i += 1;
        j += 1;
        k += 1;
    }

    false
}

fn pair_of_letters(s: &String) -> bool {
    for (i, c) in s.chars().enumerate() {
        if i < s.len() - 1 {
            let ith = s.chars().nth(i + 1).unwrap();
            let mut j = i + 2;
            let mut k = j + 1;
            while j + 1 < s.len() {
                let jth = s.chars().nth(j).unwrap();
                let kth = s.chars().nth(k).unwrap();

                if jth == c && kth == ith {
                    return true;
                }

                j += 1;
                k += 1;
            }
        }
    }
    false
}

fn main() {
    let contents = read_input("input.txt")
        .split("\n")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let mut nice_string_count = 0;
    for s in contents {
        if pair_of_letters(&s) && letter_between(&s) {
            nice_string_count += 1;
        }
    }

    println!("Nice String Count: {}", nice_string_count);
}
