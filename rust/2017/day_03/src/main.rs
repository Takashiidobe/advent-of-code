fn find(input: f64) -> i32 {
    let mut corner = 0f64;
    let mut val = 1f64;
    let mut step = 0f64;

    while val < input {
        if corner % 4f64 == 0f64 {
            step += 2f64;
        }
        val += step;
        corner += 1f64;
    }

    ((step / 2f64) + ((val - (step / 2f64)) - input).abs()) as i32
}

fn main() {
    println!("{}", find(289326_f64));
}
