mod days;
use days::{day_1, day_2, day_3};

fn main() {
    let (day_1a, day_1b) = day_1::solve();
    println!("Solution for day_1a: {day_1a}");
    println!("Solution for day_1b: {day_1b}");

    let (day_2a, day_2b) = day_2::solve();
    println!("Solution for day_2a: {day_2a}");
    println!("Solution for day_2b: {day_2b}");

    let (day_3a, day_3b) = day_3::solve();
    println!("Solution for day_3a: {day_3a}");
    println!("Solution for day_3b: {day_3b}");
}
