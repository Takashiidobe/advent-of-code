fn parse() -> Vec<String> {
    std::fs::read_to_string("../input.txt")
        .unwrap()
        .split("\n")
        .split_whitespace()
}
fn main() {
    println!("Hello, world!");
}
