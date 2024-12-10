use std::os::windows;

use day_02::check_report;

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let result: usize = input
        .lines()
        .filter(|line| {
            let levels: Vec<i32> = line
                .split_whitespace()
                .map(|value| value.parse::<i32>().unwrap())
                .collect();
            let mut report_status = check_report(&levels);
            if !report_status {
                report_status = create_possible_reports(&levels)
                    .iter()
                    .any(|report| day_02::check_report(report));
            }
            report_status
        })
        .count();
    result.to_string()
}

fn create_possible_reports(levels: &[i32]) -> Vec<Vec<i32>> {
    let mut result = vec![];
    for i in 0..levels.len() {
        let mut copy = levels.to_owned();
        copy.remove(i);
        result.push(copy);
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::process;

    #[test]
    fn test_example() {
        let result = process(
            "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9",
        );
        assert_eq!(result, "4".to_string())
    }
}
