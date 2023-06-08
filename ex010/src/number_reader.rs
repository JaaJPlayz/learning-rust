use std::io;

pub fn number_reader() -> i32 {
    let stdin = io::stdin();
    let mut input = String::new();

    stdin.read_line(&mut input).expect("Failed to read line.");
    let number: i32 = input.trim().parse().expect("Invalid number");

    return number;
}
