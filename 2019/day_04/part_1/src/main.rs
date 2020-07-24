fn two_next(d: &Vec<u32>) -> bool {
    (0..5).any(|i| match i {
        0 => (d[0] == d[1]) && (d[0] != d[2]),
        4 => (d[4] == d[5]) && (d[4] != d[3]),
        n => (d[n] == d[n + 1]) && (d[n] != d[n - 1]) && (d[n] != d[n + 2]),
    })
}

fn is_increasing(d: &Vec<u32>) -> bool {
    (0..5).all(|i| d[i] <= d[i + 1])
}

fn digits(num: u32) -> Vec<u32> {
    num.to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap())
        .collect::<Vec<_>>()
}

fn main() {
    const MIN_NUM: u32 = 356261;
    const MAX_NUM: u32 = 846303;

    let mut count = 0;

    for i in MIN_NUM..=MAX_NUM {
        if two_next(&digits(i)) == true && is_increasing(&digits(i)) == true {
            count += 1;
        }
    }

    println!("{}", count);
}

// fn day_two() {
//     for i in (1..=99) {
//         for j in (1..=99) {
//             let data = read_d(i, j).unwrap();
//             let parsed_data = parse_data(data);
//
//             if parsed_data[0] == 19690720 {
//                 println!("total = {}", 100 * i + j);
//                 break;
//             }
//         }
//     }
// }
// fn read_d(noun: u32, verb: usize) -> Result<Vec<usize>, Box<dyn Error>> {
//     let mut records = csv::ReaderBuilder::new()
//         .has_headers(false)
//         .from_path("d2.csv")?;
//
//     let mut data = Vec::new();
//
//     for record in records.records() {
//         let rec = record?;
//         data.push(rec);
//     }
//
//     let strings = &data[0];
//     let mut data: Vec<u32> = Vec::new();
//     for s in strings.iter() {
//         data.push(s.parse::<u32>().unwrap());
//     }
//
//     data[1] = noun;
//     data[2] = verb;
//
//     Ok(data)
// }
//
// fn parse_data(mut data: Vec<u32>) -> Vec<usize> {
//     let mut pos = 0;
//
//     while pos < data.len() {
//         let opcode = data[pos];
//         let first_d = data[data[(pos + 1)]];
//         let second_d = data[data[(pos + 2)]];
//         let output_index = data[(pos + 3)];
//
//         match opcode {
//             1 => data[output_index] = first_d + second_d,
//             2 => data[output_index] = first_d * second_d,
//             99 => break,
//             _ => break,
//         }
//         pos += 4;
//     }
//
//     data
// }

// fn fuel_for_fuel(fuel: i32, total: i32) -> i32 {
//     let additional_fuel = (fuel / 3) - 2;
//     if additional_fuel < 1 {
//         return total;
//     }
//     fuel_for_fuel(additional_fuel, total + additional_fuel)
// }
//
// fn challenge_one() {
//     let data = read_d("d1.txt").unwrap();
//
//     let gas_required: Vec<i32> = data.into_iter().map(|num| (num / 3) - 2).collect();
//
//     let gas_for_gas: Vec<i32> = gas_required
//         .iter()
//         .map(|&num| fuel_for_fuel(num, 0))
//         .collect();
//
//     let total = gas_required.into_iter().fold(0, |total, next| total + next);
//
//     let gas_req = gas_for_gas.into_iter().fold(0, |total, next| total + next);
//
//     println!("total: {}", total + gas_req);
// }
