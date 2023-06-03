fn main() {
    println!("{}", rect_area(1.3,2.7));
}

fn rect_area(base: f32, height: f32) -> String {
    let area: f32 = base * height;
    let side_info = format!("Side 1: {}\nSide 2: {}\nArea: {}", base, height, area);
    return side_info;
}