use std::fs;

pub mod day_01;
pub mod day_02;
pub mod day_03;
pub mod day_04;
pub mod day_05;
pub mod day_06;
pub mod day_07;
pub mod day_08;
pub mod day_09;

pub fn read_day_input(day: &str) -> String {
    let file_path = format!("inputs/{day}.txt");
    let contents = fs::read_to_string(&file_path);
    contents.unwrap_or_else(|_| panic!("Could not load: {file_path}"))
}
