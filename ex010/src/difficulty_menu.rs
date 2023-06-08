use crate::input_reader;

pub fn difficulty_menu() -> i32 {
    let title_string: &str = "Guessing Game";
    let width_to_format: &usize = &30;
    let centered_title: String = format!("{:^width$}", title_string, width = width_to_format);
    let difficulites = vec!["Easy", "Medium", "Hard", "VeryHard", "Impossible"];

    println!("-----------------------------\n{}\n-----------------------------", centered_title);

    for (index, item) in difficulites.iter().enumerate() {
        let option = index + 1;
        println!("[{}] {}", option, item);
    }

    println!("-----------------------------");
    print!("Choice: ");
    let chosen_difficulty: i32 = input_reader::number_reader();
    return chosen_difficulty;
}
