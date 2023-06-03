fn reverse(s: &str) -> String {
    s.chars().rev().collect()
}

fn main() {
    let my_string = "O lobo ama o bolo";
    let rev_string: String = reverse(my_string);
    
    if my_string.replace(" ", "").to_uppercase() == rev_string.replace(" ", "").to_uppercase() {
        println!("The string is a palindrome");
    }
    else {
        println!("The string is not a palindrome");
    }
}