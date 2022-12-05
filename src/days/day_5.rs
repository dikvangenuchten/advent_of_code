use std::cmp::min;

use itertools::Itertools;
use regex::Regex;

use crate::days::read_day_input;

pub fn solve(input: &str) -> (String, String) {
    let part_1 = solve_part_1(input);
    let part_2 = solve_part_2(input);
    (part_1, part_2)
}

fn solve_part_1(input_str: &str) -> String {
    let (crates, operations) = parse_input(input_str);
    let crates = apply_operations_p1(crates, operations);
    let mut out = vec![];
    for mut _crate in crates {
        if let Some(_crate) = _crate.pop() {
            out.push(_crate);
        }
    }
    String::from_iter(out)
}

fn solve_part_2(input_str: &str) -> String {
    let (crates, operations) = parse_input(input_str);
    let crates = apply_operations_p2(crates, operations);
    let mut out = vec![];
    for mut _crate in crates {
        if let Some(_crate) = _crate.pop() {
            out.push(_crate);
        }
    }
    String::from_iter(out)
}

fn parse_input(input_str: &str) -> (Vec<Vec<char>>, Vec<(u32, u32, u32)>) {
    let mut i = 0;
    let mut crates: Vec<Vec<char>> = vec![];

    // Parse crate stacks
    for _crate in input_str.chars().into_iter().chunks(4).into_iter() {
        match _crate.collect::<Vec<char>>()[..] {
            ['[', x, ']', end_char] => {
                match crates.get_mut(i) {
                    Some(stack) => stack.insert(0, x),
                    None => {
                        if crates.len() <= i {
                            crates.resize(i + 1, vec![]);
                        }
                        crates[i] = vec![x]
                    }
                };
                if end_char == '\n' {
                    i = 0
                } else {
                    i += 1
                }
            }
            [' ', ' ', ' ', ' '] => i += 1,
            [' ', ' ', ' ', '\n'] => i = 0,
            _ => break,
        };
    }
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let mut operations = Vec::new();
    // parse operations
    for operation in re.captures_iter(input_str) {
        operations.push((
            operation.get(1).unwrap().as_str().parse::<u32>().unwrap(),
            operation.get(2).unwrap().as_str().parse::<u32>().unwrap(),
            operation.get(3).unwrap().as_str().parse::<u32>().unwrap(),
        ))
    }
    (crates, operations)
}

fn apply_operations_p1(
    mut crates: Vec<Vec<char>>,
    operations: Vec<(u32, u32, u32)>,
) -> Vec<Vec<char>> {
    for operation in operations {
        crates = apply_operation_p1(crates, operation);
    }
    crates
}

fn apply_operation_p1(mut crates: Vec<Vec<char>>, operation: (u32, u32, u32)) -> Vec<Vec<char>> {
    for _ in 0..operation.0 as usize {
        let _crate = crates[(operation.1 - 1) as usize].pop().unwrap();
        crates[(operation.2 - 1) as usize].push(_crate);
    }
    return crates;
}

fn apply_operations_p2(
    mut crates: Vec<Vec<char>>,
    operations: Vec<(u32, u32, u32)>,
) -> Vec<Vec<char>> {
    for operation in operations {
        crates = apply_operation_p2(crates, operation);
    }
    crates
}

fn apply_operation_p2(mut crates: Vec<Vec<char>>, operation: (u32, u32, u32)) -> Vec<Vec<char>> {
    let mut intermediate_stack = vec![];
    for _ in 0..operation.0 as usize {
        let _crate = crates[(operation.1 - 1) as usize].pop().unwrap();
        intermediate_stack.push(_crate);
    }
    for _ in 0..operation.0 as usize {
        let _crate = intermediate_stack.pop().unwrap();
        crates[(operation.2 - 1) as usize].push(_crate);
    }

    return crates;
}

#[cfg(test)]
mod tests {

    use super::*;
    use rstest::*;

    #[fixture]
    fn input_str() -> String {
        read_day_input("test_day_5")
    }

    #[fixture]
    fn parsed_input() -> (Vec<Vec<char>>, Vec<(u32, u32, u32)>) {
        (
            vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']],
            vec![(1, 2, 1), (3, 1, 3), (2, 2, 1), (1, 1, 2)],
        )
    }

    #[rstest]
    fn test_parse_input(input_str: String, parsed_input: (Vec<Vec<char>>, Vec<(u32, u32, u32)>)) {
        assert_eq!(parse_input(&input_str), parsed_input)
    }

    #[rstest]
    #[case(
        vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']],
        (1, 2, 1),
        vec![vec!['Z', 'N', 'D'], vec!['M', 'C'], vec!['P']],
    )]
    #[case(
        vec![vec!['Z', 'N', 'D'], vec!['M', 'C'], vec!['P']],
        (3, 1, 3),
        vec![vec![], vec!['M', 'C'], vec!['P', 'D', 'N', 'Z']],
    )]
    #[case(
        vec![vec![], vec!['M', 'C'], vec!['P', 'D', 'N', 'Z']],
        (2, 2, 1),
        vec![vec!['C', 'M'], vec![], vec!['P', 'D', 'N', 'Z']],

    )]
    #[case(
        vec![vec!['C', 'M'], vec![], vec!['P', 'D', 'N', 'Z']],
        (1, 1, 2),
        vec![vec!['C'], vec!['M'], vec!['P', 'D', 'N', 'Z']],

    )]
    fn test_apply_operation_p1(
        #[case] crates: Vec<Vec<char>>,
        #[case] operation: (u32, u32, u32),
        #[case] crates_after: Vec<Vec<char>>,
    ) {
        assert_eq!(apply_operation_p1(crates, operation), crates_after)
    }

    #[rstest]
    #[case(
        vec![vec!['C'], vec!['M'], vec!['P', 'D', 'N', 'Z']],
    )]
    fn test_apply_operations_p1(input_str: String, #[case] expected: Vec<Vec<char>>) {
        let (crates, operations) = parse_input(&input_str);
        assert_eq!(apply_operations_p1(crates, operations), expected)
    }

    #[rstest]
    fn test_solve_part_1(input_str: String) {
        assert_eq!(solve_part_1(&input_str), "CMZ")
    }

    #[rstest]
    #[case(
        vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']],
        (1, 2, 1),
        vec![vec!['Z', 'N', 'D'], vec!['M', 'C'], vec!['P']],
    )]
    #[case(
        vec![vec!['Z', 'N', 'D'], vec!['M', 'C'], vec!['P']],
        (3, 1, 3),
        vec![vec![], vec!['M', 'C'], vec!['P', 'Z', 'N', 'D']],
    )]
    #[case(
        vec![vec![], vec!['M', 'C'], vec!['P', 'Z', 'N', 'D']],
        (2, 2, 1),
        vec![vec!['M', 'C'], vec![], vec!['P', 'Z', 'N', 'D']],

    )]
    #[case(
        vec![vec!['M', 'C'], vec![], vec!['P', 'Z', 'N', 'D']],
        (1, 1, 2),
        vec![vec!['M'], vec!['C'], vec!['P', 'Z', 'N', 'D']],

    )]
    fn test_apply_operation_p2(
        #[case] crates: Vec<Vec<char>>,
        #[case] operation: (u32, u32, u32),
        #[case] crates_after: Vec<Vec<char>>,
    ) {
        assert_eq!(apply_operation_p2(crates, operation), crates_after)
    }
}
