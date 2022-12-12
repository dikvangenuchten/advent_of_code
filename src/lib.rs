pub mod days;
use std::time::{self, Duration};

use days::{
    day_01, day_02, day_03, day_04, day_05, day_06, day_07, day_08, day_09, day_10, read_day_input,
};

pub fn run_all_days() -> Vec<Duration> {
    let start = time::Instant::now();
    let (day_1a, day_1b) = day_01::solve(&read_day_input("day_01"));
    println!("Solution for day_1a: {day_1a}");
    println!("Solution for day_1b: {day_1b}");
    let day_1_time = start.elapsed();

    let start = time::Instant::now();
    let (day_2a, day_2b) = day_02::solve(&read_day_input("day_02"));
    println!("Solution for day_2a: {day_2a}");
    println!("Solution for day_2b: {day_2b}");
    let day_2_time = start.elapsed();

    let start = time::Instant::now();
    let (day_3a, day_3b) = day_03::solve(&read_day_input("day_03"));
    println!("Solution for day_3a: {day_3a}");
    println!("Solution for day_3b: {day_3b}");
    let day_3_time = start.elapsed();

    let start = time::Instant::now();
    let (day_4a, day_4b) = day_04::solve(&read_day_input("day_04"));
    println!("Solution for day_4a: {day_4a}");
    println!("Solution for day_4b: {day_4b}");
    let day_4_time = start.elapsed();

    let start = time::Instant::now();
    let (day_5a, day_5b) = day_05::solve(&read_day_input("day_05"));
    println!("Solution for day_5a: {day_5a}");
    println!("Solution for day_5b: {day_5b}");
    let day_5_time = start.elapsed();

    let start = time::Instant::now();
    let (day_6a, day_6b) = day_06::solve(&read_day_input("day_06"));
    println!("Solution for day_6a: {day_6a}");
    println!("Solution for day_6b: {day_6b}");
    let day_6_time = start.elapsed();

    let start = time::Instant::now();
    let (day_7a, day_7b) = day_07::solve(&read_day_input("day_07"));
    println!("Solution for day_7a: {day_7a}");
    println!("Solution for day_7b: {day_7b}");
    let day_7_time = start.elapsed();

    let start = time::Instant::now();
    let (day_8a, day_8b) = day_08::solve(&read_day_input("day_08"));
    println!("Solution for day_8a: {day_8a}");
    println!("Solution for day_8b: {day_8b}");
    let day_8_time = start.elapsed();

    let start = time::Instant::now();
    let (day_9a, day_9b) = day_09::solve(&read_day_input("day_09"));
    println!("Solution for day_9a: {day_9a}");
    println!("Solution for day_9b: {day_9b}");
    let day_9_time = start.elapsed();

    let start = time::Instant::now();
    let (day_10a, day_10b) = day_10::solve(&read_day_input("day_10"));
    println!("Solution for day_10a: {day_10a}");
    println!("Solution for day_10b: \n{day_10b}");
    let day_10_time = start.elapsed();

    vec![
        day_1_time,
        day_2_time,
        day_3_time,
        day_4_time,
        day_5_time,
        day_6_time,
        day_7_time,
        day_8_time,
        day_9_time,
        day_10_time,
    ]
}
