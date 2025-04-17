use std::{
    fs::OpenOptions,
    io::{BufWriter, Write},
    iter::successors,
    ops::{Div, Rem},
};

// TODO: finish implementing optional writing to a file (i.e. figure out the borrow shit)
fn main() {
    let input = include_str!("./input.txt");
    let file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open("part2_filtered.txt")
        .unwrap();
    let mut buf_writer = BufWriter::new(file);
    let output = process(input, &mut buf_writer);
    buf_writer.flush().expect("Writer failed for some reason");
    dbg!(output);
}

enum Operator {
    Add,
    Mult,
    Concat, // Has precedence over Add and Muilt -> needs to be evaluated last
}

fn process(input: &str, writer: &mut impl Write) -> String {
    let mut result = 0u128;

    for line in input.lines() {
        let (left, right) = line.split_once(':').expect("Invalid line: no ':'!");
        let test_value = left.parse::<u128>().expect("Conversion Error");
        let equation_values: Vec<u128> = right
            .trim()
            .split(' ')
            .map(|str_value| str_value.parse::<u128>().expect("Conversion Error"))
            .collect();

        if init_dfs(&equation_values, test_value) {
            result += test_value;
        }

        writer.write_fmt(format_args!("{}\n", line)).unwrap();
    }

    result.to_string()
}

#[allow(dead_code)]
fn test_process(input: &str) -> String {
    process(input, &mut std::io::empty())
}
fn init_dfs(equation_values: &[u128], test_value: u128) -> bool {
    solve_dfs(equation_values, Operator::Add, test_value)
        || solve_dfs(equation_values, Operator::Mult, test_value)
        || solve_dfs(equation_values, Operator::Concat, test_value)
}

/// Tries to solve the equation right to left
/// This should work well because the || operator is evaluated left to right
fn solve_dfs(lhs: &[u128], operator: Operator, rem_test_value: u128) -> bool {
    // if one side of the equation is already too large -> prune this branch
    if lhs.len() == 1 {
        return lhs[0] == rem_test_value;
    }
    let rhs = lhs.last().expect("lhs should not be empty!");
    // reverse operations on the test_value and see if you get below 0 (or invalid into uneven values)
    let new_rem_test_value = match operator {
        // straight forward
        Operator::Add => rem_test_value.checked_sub(*rhs),
        // mult has to resolve properly for valid combination -> check result and remainder
        Operator::Mult => {
            let result = rem_test_value.div(*rhs);
            let remainder = rem_test_value.rem(*rhs);
            match result {
                0 => None,
                _ if remainder != 0 => None,
                _ => Some(result),
            }
        }
        Operator::Concat => {
            // first reverse the addition of the actual element
            let n_digits = count_digits(*rhs);
            let result = rem_test_value.checked_sub(*rhs);
            // if that's fine then reverse the mult that created the additional 0's added to the lhs
            result.and_then(|value| {
                let result = value.div(n_digits * 10);
                let remainder = value.rem(n_digits * 10);
                match result {
                    0 => None,
                    _ if remainder != 0 => None,
                    _ => Some(result),
                }
            })
        }
    };

    // if the above operation resulted in an invalid test value -> abort this branch
    if new_rem_test_value.is_none() {
        return false;
    }

    let last_index = lhs.len() - 1;
    solve_dfs(
        &lhs[..last_index],
        Operator::Add,
        new_rem_test_value.unwrap(),
    ) || solve_dfs(
        &lhs[..last_index],
        Operator::Mult,
        new_rem_test_value.unwrap(),
    ) || solve_dfs(
        &lhs[..last_index],
        Operator::Concat,
        new_rem_test_value.unwrap(),
    )
}

fn solve(
    current_value: u128,
    remaining_equation: &[u128],
    operator: Operator,
    test_value: u128,
) -> bool {
    if remaining_equation.is_empty() {
        current_value == test_value
    } else {
        let new_value = match operator {
            Operator::Add => current_value + remaining_equation[0],
            Operator::Mult => current_value * remaining_equation[0],
            Operator::Concat => {
                // determine number of digits for the remaining equation
                let n_digits = count_digits(remaining_equation[0]);
                current_value * n_digits * 10 + remaining_equation[0]
            }
        };
        solve(
            new_value,
            &remaining_equation[1..],
            Operator::Add,
            test_value,
        ) || solve(
            new_value,
            &remaining_equation[1..],
            Operator::Mult,
            test_value,
        ) || solve(
            new_value,
            &remaining_equation[1..],
            Operator::Concat,
            test_value,
        )
    }
}

fn count_digits(n: u128) -> u128 {
    (successors(Some(n), |&n| (n >= 10).then_some(n / 10)).count()) as u128
}

#[cfg(test)]
mod tests {

    use crate::{count_digits, test_process};

    #[test]
    fn test_example() {
        let result = test_process(
            "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20",
        );
        assert_eq!(result, "11387".to_string())
    }

    #[test]
    fn test_egde_cases() {
        assert_eq!(test_process("11: 1 1"), "11".to_string());
        assert_eq!(test_process("999: 9 9 9"), "999".to_string());
        assert_eq!(test_process("100001: 10000 1"), "100001".to_string());
        assert_eq!(
            test_process("1021209310293: 102120931029 3"),
            "1021209310293".to_string()
        );
        assert_eq!(test_process("11: 1 1"), "11".to_string());
    }

    #[test]
    fn test_count_digits() {
        assert_eq!(count_digits(1), 1);
        assert_eq!(count_digits(0), 1);
        assert_eq!(count_digits(10), 2);
        assert_eq!(count_digits(23452), 5);
        assert_eq!(count_digits(1238195470918), 13);
    }
}
