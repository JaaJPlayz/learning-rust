mod number_reader;
mod rng_for_cpu;
mod difficulty_menu;
mod input_reader;
fn main() {
    // let player_number: i32 = number_reader::number_reader();
    // println!("{:?}", rng_for_cpu::rng_for_cpu());
    let idk: i32 = difficulty_menu::difficulty_menu();
    println!("{}", idk);
}
