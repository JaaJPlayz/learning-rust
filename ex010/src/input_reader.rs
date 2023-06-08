use std::io;

pub fn string_reader() -> String {
    let mut string_input = String::new();
    let stdin = io::stdin();
    let _ = stdin.read_line(&mut string_input);

    return string_input;
}

pub fn number_reader() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    input.trim().parse().expect("Failed to parse input as number")
}
