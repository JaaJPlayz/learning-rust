use std::io::{ self, Write };

pub fn user_input(prompt: &str) -> i32 {
    print!("{}", prompt);
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).expect("Failed to read line");
    let x: i32 = input_line.trim().parse().expect("Input not an integer");
    return x;
}

pub(crate) fn main() {
    let user_response = user_input("Please enter a number: ");
    println!("The number you entered is: {:?}", user_response);
}
