pub struct Machine {
    pub button_a: (u32, u32),
    pub button_b: (u32, u32),
    pub prize: (u32, u32),
}

pub fn parse_machines(input: &str) -> Vec<Machine> {
    input
        .split("\n\n")
        .map(|machine| {
            let mut machine_iter = machine.lines();
            Machine {
                button_a: parse_button(machine_iter.next().unwrap()),
                button_b: parse_button(machine_iter.next().unwrap()),
                prize: parse_prize(machine_iter.next().unwrap()),
            }
        })
        .collect()
}

fn parse_button(line: &str) -> (u32, u32) {
    line[12..]
        .split_once(", Y+")
        .map(|(x, y)| (x.parse::<u32>().unwrap(), y.parse::<u32>().unwrap()))
        .unwrap()
}

fn parse_prize(line: &str) -> (u32, u32) {
    line[9..]
        .split_once(", Y=")
        .map(|(x, y)| (x.parse::<u32>().unwrap(), y.parse::<u32>().unwrap()))
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::parse_button;

    #[test]
    fn test_button_parsing() {
        let button = parse_button("Button A: X+94, Y+34");
        assert_eq!(button, (94, 34));
        let button = parse_button("Button B: X+22, Y+67");
        assert_eq!(button, (22, 67));
    }
}
