use crate::days::read_day_input;

pub fn solve() -> (u32, u32) {
    let input = read_day_input("day_2");
    let part_1 = solve_part_1(&input);
    let part_2 = solve_part_2(&input);
    return (part_1, part_2);
}

fn solve_part_1(input_str: &str) -> u32 {
    input_str
        .split("\n")
        .map(|round_str| calculate_round_score_part_1(round_str))
        .sum()
}

fn solve_part_2(input_str: &str) -> u32 {
    input_str
        .split("\n")
        .map(|round_str| calculate_round_score_part_2(round_str))
        .sum()
}

fn calculate_round_score_part_1(round_str: &str) -> u32 {
    match round_str {
        // Win
        "A Y" => 6 + 2,
        "B Z" => 6 + 3,
        "C X" => 6 + 1,
        // Draw
        "A X" => 3 + 1,
        "B Y" => 3 + 2,
        "C Z" => 3 + 3,
        // Lose
        "A Z" => 0 + 3,
        "B X" => 0 + 1,
        "C Y" => 0 + 2,
        _ => unreachable!("Invalid input"),
    }
}

fn calculate_round_score_part_2(round_str: &str) -> u32 {
    match round_str {
        // Win
        "A Y" => 3 + 1,
        "B Z" => 6 + 3,
        "C X" => 0 + 2,
        // Draw
        "A X" => 0 + 3,
        "B Y" => 3 + 2,
        "C Z" => 6 + 1,
        // Lose
        "A Z" => 6 + 2,
        "B X" => 0 + 1,
        "C Y" => 3 + 3,
        _ => unreachable!("Invalid input"),
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use rstest::*;

    #[test]
    fn test_read_day_input() {
        read_day_input("day_2");
    }

    #[rstest]
    #[case("A Y\nB X\nC Z")]
    fn test_day_2p1(#[case] input_str: &str) {
        assert_eq!(solve_part_1(input_str), 15)
    }

    #[rstest]
    #[case("A Y\nB X\nC Z")]
    fn test_day_2p2(#[case] input_str: &str) {
        assert_eq!(solve_part_2(input_str), 12)
    }

    #[rstest]
    #[case("A Y", 8)]
    #[case("B X", 1)]
    #[case("C Z", 6)]
    fn test_round_scoring(#[case] round_str: &str, #[case] expected_score: u32) {
        assert_eq!(calculate_round_score_part_1(round_str), expected_score);
    }

    #[rstest]
    #[case("A Y", 4)]
    #[case("B X", 1)]
    #[case("C Z", 7)]
    fn test_round_scoring_p2(#[case] round_str: &str, #[case] expected_score: u32) {
        assert_eq!(calculate_round_score_part_2(round_str), expected_score);
    }
}
