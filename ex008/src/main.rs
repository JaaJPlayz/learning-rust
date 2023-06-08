pub mod foo;

use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let my_random_number: i32 = rng.gen_range(0..10);
    let my_random_negative_number = rng.gen_range(-10..0);

    println!("{}", my_random_number);
    println!("{}", my_random_negative_number);

    foo::foo();
}
