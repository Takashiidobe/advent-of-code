use regex::Regex;
use std::fs::read_to_string;

#[macro_use]
extern crate lazy_static;

#[derive(Debug, Clone)]
struct Passport {
    birth_year: Option<u32>,
    issue_year: Option<u32>,
    expiration_year: Option<u32>,
    height: Option<String>,
    hair_color: Option<String>,
    eye_color: Option<String>,
    passport_id: Option<String>,
}

lazy_static! {
    static ref CAPTURE_RE: Regex = Regex::new(r"^([a-zA-Z]+):(.*)$").unwrap();
    static ref HAIR_COLOR_RE: Regex = Regex::new("^#[0-9a-f]{6}$").unwrap();
    static ref PASSPORT_ID_RE: Regex = Regex::new("^[0-9]{9}$").unwrap();
}

impl Passport {
    fn new() -> Passport {
        Passport {
            birth_year: None,
            issue_year: None,
            expiration_year: None,
            height: None,
            hair_color: None,
            eye_color: None,
            passport_id: None,
        }
    }
    fn is_valid(&self) -> bool {
        self.birth_year.is_some()
            && self.issue_year.is_some()
            && self.expiration_year.is_some()
            && self.height.is_some()
            && self.hair_color.is_some()
            && self.eye_color.is_some()
            && self.passport_id.is_some()
    }
    fn is_valid2(&self) -> bool {
        self.valid_birth_year()
            && self.valid_issue_year()
            && self.valid_expiration_year()
            && self.valid_height()
            && self.valid_hair_color()
            && self.valid_eye_color()
            && self.valid_passport_id()
    }
    fn valid_birth_year(&self) -> bool {
        if self.birth_year.is_none() {
            return false;
        }
        let birth_year = self.birth_year.unwrap();
        birth_year >= 1920 && birth_year <= 2002
    }
    fn valid_issue_year(&self) -> bool {
        if self.issue_year.is_none() {
            return false;
        }
        let issue_year = self.issue_year.unwrap();
        issue_year >= 2010 && issue_year <= 2020
    }
    fn valid_expiration_year(&self) -> bool {
        if self.expiration_year.is_none() {
            return false;
        }
        let expiration_year = self.expiration_year.unwrap();
        expiration_year >= 2020 && expiration_year <= 2030
    }
    fn valid_height(&self) -> bool {
        if self.height.is_none() {
            return false;
        }
        let chars = self.height.clone().unwrap();
        let val: usize = chars[0..chars.len() - 2].parse().unwrap_or(0);
        let unit: &str = &chars[chars.len() - 2..chars.len()];
        if unit == "cm" {
            return 150 <= val && val <= 193;
        }
        if unit == "in" {
            return 59 <= val && val <= 76;
        }
        false
    }
    fn valid_hair_color(&self) -> bool {
        if self.hair_color.is_none() {
            return false;
        }
        if HAIR_COLOR_RE.is_match(self.hair_color.clone().unwrap().as_str()) {
            return true;
        }
        false
    }
    fn valid_eye_color(&self) -> bool {
        if self.eye_color.is_none() {
            return false;
        }
        match self.eye_color.clone().unwrap().as_str() {
            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
            _ => false,
        }
    }
    fn valid_passport_id(&self) -> bool {
        if self.passport_id.is_none() {
            return false;
        }
        if PASSPORT_ID_RE.is_match(self.passport_id.clone().unwrap().as_str()) {
            return true;
        }
        false
    }
}

fn parse() -> String {
    read_to_string("../input.txt").unwrap()
}

fn captures() -> Vec<Vec<(String, String)>> {
    let input = parse();
    let split_input = input.split('\n');
    let mut captures = vec![];
    let mut v = vec![];
    for line in split_input {
        if line.len() != 0 {
            let split_line: Vec<String> = line.split(' ').map(|s| s.to_string()).collect();
            for field in split_line {
                let captured = CAPTURE_RE.captures(&field).unwrap();
                match &captured[1] {
                    "byr" => v.push(("byr".to_string(), captured[2].to_string())),
                    "ecl" => v.push(("ecl".to_string(), captured[2].to_string())),
                    "eyr" => v.push(("eyr".to_string(), captured[2].to_string())),
                    "hcl" => v.push(("hcl".to_string(), captured[2].to_string())),
                    "hgt" => v.push(("hgt".to_string(), captured[2].to_string())),
                    "iyr" => v.push(("iyr".to_string(), captured[2].to_string())),
                    "pid" => v.push(("pid".to_string(), captured[2].to_string())),
                    _ => {}
                }
            }
        } else {
            captures.push(v);
            v = vec![];
        }
    }
    captures
}

fn parse_passports() -> Vec<Passport> {
    let captures = captures();
    let mut v = vec![];
    for fields in captures {
        let mut p = Passport::new();
        for field in fields {
            match field.0.as_str() {
                "byr" => p.birth_year = Some(field.1.parse::<u32>().unwrap()),
                "iyr" => p.issue_year = Some(field.1.parse::<u32>().unwrap()),
                "eyr" => p.expiration_year = Some(field.1.parse::<u32>().unwrap()),
                "hgt" => p.height = Some(field.1),
                "hcl" => p.hair_color = Some(field.1),
                "ecl" => p.eye_color = Some(field.1),
                "pid" => p.passport_id = Some(field.1),
                _ => {}
            }
        }
        v.push(p);
    }
    v
}

fn part_one() -> u32 {
    let mut count = 0;
    let passports = parse_passports();
    for passport in passports {
        if passport.is_valid() {
            count += 1;
        }
    }
    count
}

fn part_two() -> u32 {
    let mut count = 0;
    let passports = parse_passports();
    for passport in passports {
        if passport.is_valid2() {
            count += 1;
        }
    }

    count
}

fn main() {
    println!("part one is: {}", part_one());
    println!("part two is: {}", part_two());
}
