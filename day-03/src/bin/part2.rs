use regex::Regex;
fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let re = Regex::new(r"mul\([1-9][0-9]{0,2},[1-9][0-9]{0,2}\)|do\(\)|don't\(\)").unwrap();

    let multiplications: Vec<&str> = re.find_iter(input).map(|m| m.as_str()).collect();

    let mut result = 0;
    let mut should_add = true;

    for instruction in multiplications {
        match instruction {
            "do()" => should_add = true,
            "don't()" => should_add = false,
            _ if should_add => {
                let mul_str = instruction.trim_start_matches("mul(");
                let mul_str = mul_str.trim_end_matches(")");
                let (l, r) = mul_str.split_once(',').unwrap();

                result += l.parse::<u32>().unwrap() * r.parse::<u32>().unwrap()
            }
            _ => {}
        }
    }

    result.to_string()
}

#[cfg(test)]
mod tests {
    use crate::process;

    #[test]
    fn test_example() {
        let result =
            process("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))");
        assert_eq!(result, "48".to_string())
    }
}
