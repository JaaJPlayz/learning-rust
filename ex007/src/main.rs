fn main() {
    let my_tuple = (1, 2, -3, 8, 5, -10);
    println!("Tell me about {:?}", my_tuple);
    match my_tuple {
        (1, ..) => println!("The tuple starts with one."),
        (.., 10) => println!("The tuple ends with ten."),
        _ => println!("Element not found."),
    }
}
