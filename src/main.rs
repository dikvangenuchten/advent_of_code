mod days;
use days::{day_1, day_2};

fn main() {
    let (day_1a, day_1b) = day_1::solve();
    println!("Solution for day_1a: {day_1a}");
    println!("Solution for day_1b: {day_1b}");

    let (day_1a, day_1b) = day_2::solve();
    println!("Solution for day_1a: {day_1a}");
    println!("Solution for day_1b: {day_1b}");
}
