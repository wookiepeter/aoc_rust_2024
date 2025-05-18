use day_17::ProgramState;
fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let mut program_state = ProgramState::parse(input);

    println!("{:?}", program_state);

    program_state.run();

    println!("{:?}", program_state);

    let output: Vec<String> = program_state
        .output
        .iter()
        .map(|value| value.to_string())
        .collect();

    output.join(",")
}

#[cfg(test)]
mod tests {
    use crate::process;

    #[test]
    fn test_example() {
        let result = process(
            "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0",
        );
        assert_eq!(result, "4,6,3,5,6,3,5,2,1,0".to_string())
    }
}
