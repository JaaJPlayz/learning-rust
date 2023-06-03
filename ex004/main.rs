fn main() {
    let test: String = fizz_buzz(10);
    println!("{}", test);
}

fn fizz_buzz(value: i32) -> String {
    if value % 3 == 0 && value % 5 == 0 {
        return "FizzBuzz".to_string();
    } else {
        if value % 3 == 0 {
            return "Fizz".to_string();
        } else if value % 5 == 0 {
            return "Buzz".to_string();
        } else {
            return value.to_string();
        }
    }
}
