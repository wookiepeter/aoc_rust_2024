use regex::Regex;

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let re = Regex::new(r"mul\([1-9][0-9]{0,2},[1-9][0-9]{0,2}\)").unwrap();

    let multiplications: Vec<&str> = re.find_iter(input).map(|m| m.as_str()).collect();

    let result: u32 = multiplications
        .iter()
        .map(|mul_str| {
            let mul_str = mul_str.trim_start_matches("mul(");
            let mul_str = mul_str.trim_end_matches(")");
            let (l, r) = mul_str.split_once(',').unwrap();

            l.parse::<u32>().unwrap() * r.parse::<u32>().unwrap()
        })
        .sum();

    result.to_string()
}

#[cfg(test)]
mod tests {
    use crate::process;

    #[test]
    fn test_example() {
        let result =
            process("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))");
        assert_eq!(result, "161".to_string())
    }
}
