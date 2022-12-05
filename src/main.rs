mod days;
use days::{day_1, day_2, day_3, day_4, day_5};

use crate::days::read_day_input;

fn main() {
    let (day_1a, day_1b) = day_1::solve(&read_day_input("day_1"));
    println!("Solution for day_1a: {day_1a}");
    println!("Solution for day_1b: {day_1b}");

    let (day_2a, day_2b) = day_2::solve(&read_day_input("day_2"));
    println!("Solution for day_2a: {day_2a}");
    println!("Solution for day_2b: {day_2b}");

    let (day_3a, day_3b) = day_3::solve(&read_day_input("day_3"));
    println!("Solution for day_3a: {day_3a}");
    println!("Solution for day_3b: {day_3b}");

    let (day_4a, day_4b) = day_4::solve(&read_day_input("day_4"));
    println!("Solution for day_4a: {day_4a}");
    println!("Solution for day_4b: {day_4b}");

    let (day_5a, day_5b) = day_5::solve(&read_day_input("day_5"));
    println!("Solution for day_5a: {day_5a}");
    println!("Solution for day_5b: {day_5b}");
}
