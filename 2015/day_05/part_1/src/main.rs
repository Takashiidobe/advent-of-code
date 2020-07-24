use std::collections::HashSet;
use std::fs::read_to_string;

fn read_input(file_name: &str) -> String {
    read_to_string(file_name).unwrap()
}

fn string_contains_three_vowels(s: &String) -> bool {
    let mut vowel_count = 0;
    let mut set = HashSet::new();
    set.insert('a');
    set.insert('e');
    set.insert('i');
    set.insert('o');
    set.insert('u');

    let mut i = 0;
    while i < s.len() {
        let ith_char = s.chars().nth(i).unwrap();
        if set.contains(&ith_char) {
            vowel_count += 1;
        }
        i += 1;
    }
    vowel_count >= 3
}

fn string_has_duplicates(s: &String) -> bool {
    let mut i = 0;
    let mut j = 1;

    while j < s.len() {
        let ith_char = s.chars().nth(i).unwrap();
        let jth_char = s.chars().nth(j).unwrap();
        if ith_char == jth_char {
            return true;
        }
        i += 1;
        j += 1;
    }
    false
}

fn string_doesnt_have(s: &String) -> bool {
    let mut set = HashSet::new();

    set.insert("ab".to_string());
    set.insert("cd".to_string());
    set.insert("pq".to_string());
    set.insert("xy".to_string());

    let mut i = 0;
    let mut j = 1;

    while j < s.len() {
        let ith = s.chars().nth(i).unwrap();
        let jth = s.chars().nth(j).unwrap();
        let pair: String = vec![ith, jth].into_iter().collect();
        if set.contains(&pair) {
            return false;
        }
        i += 1;
        j += 1;
    }
    true
}

fn main() {
    let contents = read_input("input.txt")
        .split("\n")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let mut nice_string_count = 0;
    for s in contents {
        if string_contains_three_vowels(&s) && string_doesnt_have(&s) && string_has_duplicates(&s) {
            nice_string_count += 1;
        }
    }

    println!("Nice String Count: {}", nice_string_count);
}
