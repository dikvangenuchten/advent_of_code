use std::{collections::BinaryHeap, fs};

pub fn solve() -> (u32, u32) {
    let input = read_day_input("day_1");
    let part_1 = solve_part_1(&input);
    let part_2 = solve_part_2(&input);
    return (part_1, part_2);
}

fn solve_part_1(input_str: &str) -> u32 {
    let max_calorie = calories_per_elf(input_str)
        .max()
        .expect("There should be atleast one value");
    return max_calorie;
}

fn solve_part_2(input_str: &str) -> u32 {
    let mut calories_per_elf = calories_per_elf(input_str).collect::<BinaryHeap<u32>>();
    return (0..3)
        .map(|_| {
            calories_per_elf
                .pop()
                .expect("Should contain more then 3 elfs worth")
        })
        .sum();
}

fn calories_per_elf(input_str: &str) -> impl Iterator<Item = u32> + '_ {
    input_str.split("\n\n").map(|elf_str| {
        elf_str
            .split("\n")
            .map(|food| {
                food.parse::<u32>()
                    .expect("string should be parseable as int")
            })
            .sum::<u32>()
    })
}

fn read_day_input(day: &str) -> String {
    let file_path = format!("inputs/{day}.txt");
    let contents = fs::read_to_string(&file_path);
    contents.expect(&format!("Could not load: {file_path}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[test]
    fn test_read_day_input() {
        read_day_input("day_1");
    }

    #[rstest]
    #[case("1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000")]
    fn test_day_1p1(#[case] input_str: &str) {
        assert_eq!(solve_part_1(input_str), 24000)
    }

    #[rstest]
    #[case("1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000")]
    fn test_day_1p2(#[case] input_str: &str) {
        assert_eq!(solve_part_2(input_str), 45000)
    }

    #[rstest]
    #[case("1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000")]
    fn test_calories_per_elf(#[case] input_str: &str) {
        assert_eq!(
            vec![6000, 4000, 11000, 24000, 10000],
            calories_per_elf(input_str).collect::<Vec<u32>>(),
        );
    }
}
