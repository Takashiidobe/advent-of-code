use md5;
use std::fs::File;
use std::io::Read;
use std::io::Result;

fn read_input(file_name: &str) -> Result<String> {
    let mut f = File::open(file_name)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;

    Ok(s)
}

fn check_md5(s: &str) -> Option<String> {
    let digest = md5::compute(s);

    let digest = String::from_utf8(digest.to_vec()).unwrap();

    let slice = &digest[..5];

    if slice == "00000" {
        return Some(slice.to_string());
    }

    None
}

fn main() {
    let input = read_input("input4.txt").unwrap();

    let mut counter: i32 = 0;

    while counter < 10000 {
        let string_count = counter.to_string();

        let final_string = input + &string_count.clone();
        if check_md5(&final_string).is_some() {
            println!("{}", &final_string);
        }
        counter += 1;
    }
}
// day 1
// use std::error::Error;
// use std::fs::File;
// use std::io;
// use std::io::Read;
// use std::io::Result;
//
// fn read_input(file_name: &str) -> Result<String> {
//     let mut f = File::open(file_name)?;
//     let mut s = String::new();
//     f.read_to_string(&mut s)?;
//
//     Ok(s)
// }
//
// fn floor_from_string(s: String) -> i32 {
//     let mut floor = 0;
//     for (pos, c) in s.char_indices() {
//         match c {
//             '(' => floor += 1,
//             ')' => floor -= 1,
//             _ => {}
//         }
//         if floor == -1 {
//             return (pos + 1) as i32;
//         }
//     }
//     0
// }
//
// fn day_one() {
//     let s = read_input("input1.txt").unwrap();
//     let floor = floor_from_string(s);
//
//    println!("{}", floor);
// }

// day2
// use std::error::Error;
// use std::fs::File;
// use std::io;
// use std::io::Read;
// use std::io::Result;
//
// fn read_input(file_name: &str) -> Result<String> {
//     let mut f = File::open(file_name)?;
//     let mut s = String::new();
//
//     f.read_to_string(&mut s)?;
//
//     Ok(s)
// }
//
// fn input_to_feet(input: String) -> usize {
//     let mut feet = 0;
//     for i in input.lines() {
//         let split_i = i.split("x").collect::<Vec<_>>();
//
//         let l = split_i[0].parse::<usize>().unwrap();
//         let w = split_i[1].parse::<usize>().unwrap();
//         let h = split_i[2].parse::<usize>().unwrap();
//
//         let ribbon_area = l * w * h;
//
//         let sides = vec![l, w, h];
//
//         let min_side = sides.iter().min().unwrap();
//         let max_side = sides.iter().max().unwrap();
//         let middle_side = sides
//             .iter()
//             .filter(|&side| side == min_side || side = max_side)
//             .collect::<Vec<_>>()[0];
//
//         let perimeter = (middle_side * 2) + (min_side * 2);
//
//         feet += ribbon_area + perimeter;
//     }
//
//     feet
// }
//
// fn day_2() {
//     let input = read_input("input2.txt").unwrap();
//     let feet = input_to_feet(input);
//
//     println!("{}", feet);
// }

// day 3
// use std::collections::HashSet;
// use std::fs::File;
// use std::io::Read;
// use std::io::Result;

// fn read_input(file_name: &str) -> Result<String> {
//     let mut f = File::open(file_name)?;
//     let mut s = String::new();
//     f.read_to_string(&mut s)?;

//     Ok(s)
// }

// fn day_3() {
//     let input = read_input("input3.txt").unwrap();

//     let mut present_map: HashSet<(i32, i32)> = HashSet::new();

//     let mut santas_location: (i32, i32) = (0, 0);

//     let mut robo_location: (i32, i32) = (0, 0);

//     present_map.insert((0, 0));

//     for (index, i) in input.char_indices() {
//         match index % 2 == 0 {
//             true => {
//                 match i {
//                     '>' => santas_location.0 += 1,
//                     '<' => santas_location.0 -= 1,
//                     '^' => santas_location.1 += 1,
//                     'v' => santas_location.1 -= 1,
//                     _ => {}
//                 }
//                 if !present_map.contains(&santas_location) {
//                     present_map.insert(santas_location);
//                 }
//             }
//             false => {
//                 match i {
//                     '>' => robo_location.0 += 1,
//                     '<' => robo_location.0 -= 1,
//                     '^' => robo_location.1 += 1,
//                     'v' => robo_location.1 -= 1,
//                     _ => {}
//                 }
//                 if !present_map.contains(&robo_location) {
//                     present_map.insert(robo_location);
//                 }
//             }
//         }
//     }

//     let houses_visited = present_map.len();

//     println!("{}", houses_visited);
// }
