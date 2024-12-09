fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let (mut a, mut b): (Vec<u32>, Vec<u32>) = input
        .lines()
        .map(|line| {
            let nums: Vec<&str> = line.split_whitespace().collect();
            (
                nums[0].parse::<u32>().unwrap(),
                nums[1].parse::<u32>().unwrap(),
            )
        })
        .unzip();

    a.sort();
    b.sort();

    let result: u32 = a
        .iter()
        .map(|a| {
            let count = b.iter().filter(|value| *value == a).count();
            (count as u32) * a
        })
        .sum();

    result.to_string()
}

#[cfg(test)]
mod tests {
    use crate::process;

    #[test]
    fn test_example() {
        let result = process(
            "3   4
4   3
2   5
1   3
3   9
3   3",
        );
        assert_eq!(result, "31".to_string())
    }
}
