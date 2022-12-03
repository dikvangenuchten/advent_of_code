use std::fs;

pub mod day_1;
pub mod day_2;
pub mod day_3;

pub fn read_day_input(day: &str) -> String {
    let file_path = format!("inputs/{day}.txt");
    let contents = fs::read_to_string(&file_path);
    contents.unwrap_or_else(|_| panic!("Could not load: {file_path}"))
}
