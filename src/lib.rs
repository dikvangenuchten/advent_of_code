mod days;

pub use days::day_1;
use std::fs;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn read_day_input(day: &str) -> String {
    let file_path = format!("inputs/{day}.txt");
    let contents = fs::read_to_string(&file_path);
    contents.expect(&format!("Could not load: {file_path}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
