fn main() {
    let number = 10;
    print!("Tell me about {}: ", number);
    match number {
        1 => println!("It's One!"),
        2 | 3 | 5 | 7 | 11 | 13 => println!("It's a prime number"),
        _ => println!("{:?} is a common or too big of a number", number),
    }

    let my_boolean = false;
    let my_binary = match my_boolean {
        true => 1,
        false => 0,
    };
    println!("Your binary is equivalent to {:?}", my_binary);
}
