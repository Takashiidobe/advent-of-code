use md5;

fn calculate_hash() -> i32 {
    let input = "ckczppom".to_owned();

    let mut number = 1;
    loop {
        let output: String = format!("{}{}", input, number);

        let h = format!("{:x}", md5::compute(output.as_bytes()));
        if h.starts_with("000000") {
            return number;
        }
        number += 1;
    }
}

fn main() {
    assert_eq!(calculate_hash(), 3938038);
    println!("{}", calculate_hash());
}
