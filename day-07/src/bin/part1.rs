fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

enum Operator {
    Add,
    Mult,
}

fn process(input: &str) -> String {
    let mut result = 0u64;
    for line in input.lines() {
        let (left, right) = line.split_once(':').expect("Invalid line: no ':'!");
        let test_value = left.parse::<u64>().expect("Conversion Error");
        let equation_values: Vec<u64> = right
            .trim()
            .split(' ')
            .map(|str_value| str_value.parse::<u64>().expect("Conversion Error"))
            .collect();

        if equation_is_solvable(&equation_values, test_value) {
            result += test_value;
        }
    }

    result.to_string()
}

// just to declutter main loop
fn equation_is_solvable(equation_values: &[u64], test_value: u64) -> bool {
    solve(
        equation_values[0],
        &equation_values[1..],
        Operator::Add,
        test_value,
    ) || solve(
        equation_values[0],
        &equation_values[1..],
        Operator::Mult,
        test_value,
    )
}

fn solve(
    current_value: u64,
    remaining_equation: &[u64],
    operator: Operator,
    test_value: u64,
) -> bool {
    if remaining_equation.is_empty() {
        current_value == test_value
    } else {
        let new_value = match operator {
            Operator::Add => current_value + remaining_equation[0],
            Operator::Mult => current_value * remaining_equation[0],
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
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::process;

    #[test]
    fn test_example() {
        let result = process(
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
        assert_eq!(result, "3749".to_string())
    }
}
