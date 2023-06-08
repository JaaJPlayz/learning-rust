use rand::Rng;

pub fn rng_for_cpu(difficulty: &str) -> Result<i32, String> {
    let mut rng = rand::thread_rng();

    match difficulty {
        "Easy" => Ok(rng.gen_range(0..=10)),
        "Medium" => Ok(rng.gen_range(0..=20)),
        "Hard" => Ok(rng.gen_range(0..=35)),
        "VeryHard" => Ok(rng.gen_range(0..=50)),
        "Impossible" => Ok(rng.gen_range(0..=100)),
        _ => Err(format!("There is no difficulty for {}", difficulty)),
    }
}
