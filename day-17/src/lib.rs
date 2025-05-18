use std::ops::BitXor;

#[derive(Debug)]
pub struct ProgramState {
    pointer: usize,
    reg_a: u32,
    reg_b: u32,
    reg_c: u32,

    program: Vec<u32>,
    pub output: Vec<u32>,
}

impl ProgramState {
    pub fn parse(input: &str) -> ProgramState {
        let mut lines = input.lines();
        let reg_a = lines
            .next()
            .unwrap()
            .trim_start_matches("Register A: ")
            .parse::<u32>()
            .unwrap();
        let reg_b = lines
            .next()
            .unwrap()
            .trim_start_matches("Register B: ")
            .parse::<u32>()
            .unwrap();
        let reg_c = lines
            .next()
            .unwrap()
            .trim_start_matches("Register C: ")
            .parse::<u32>()
            .unwrap();

        let program = lines
            .last()
            .unwrap()
            .trim_start_matches("Program: ")
            .split(',')
            .map(|s| s.parse::<u32>().unwrap())
            .collect();

        ProgramState {
            pointer: 0,
            reg_a,
            reg_b,
            reg_c,
            program,
            output: vec![],
        }
    }

    pub fn run(&mut self) {
        while self.pointer < self.program.len() {
            match self.program[self.pointer] {
                0 => self.adv(),
                1 => self.bxl(),
                2 => self.bst(),
                3 => self.jnz(),
                4 => self.bxc(),
                5 => self.out(),
                6 => self.bdv(),
                7 => self.cdv(),
                _ => panic!("Invalid OP-Code: {}", self.pointer),
            }
        }
    }

    fn combo_operand(&self) -> u32 {
        match self.program[self.pointer + 1] {
            operand if operand < 4 => operand,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            invalid_operand => panic!("Invalid operand: {invalid_operand}"),
        }
    }

    fn literal_operand(&self) -> u32 {
        self.program[self.pointer + 1]
    }

    fn adv(&mut self) {
        self.reg_a /= 2u32.pow(self.combo_operand());
        self.pointer += 2;
    }

    fn bdv(&mut self) {
        self.reg_b = self.reg_a / 2u32.pow(self.combo_operand());
        self.pointer += 2;
    }

    fn cdv(&mut self) {
        self.reg_c = self.reg_a / 2u32.pow(self.combo_operand());
        self.pointer += 2;
    }

    fn out(&mut self) {
        self.output.push(self.combo_operand() % 8);
        self.pointer += 2;
    }

    fn bxl(&mut self) {
        self.reg_b = self.reg_b.bitxor(self.literal_operand());
        self.pointer += 2;
    }

    fn bst(&mut self) {
        self.reg_b = self.combo_operand() % 8u32;
        self.pointer += 2;
    }

    fn jnz(&mut self) {
        if self.reg_a == 0 {
            self.pointer += 2;
        } else {
            self.pointer = self.literal_operand() as usize;
        }
    }

    fn bxc(&mut self) {
        self.reg_b = self.reg_b.bitxor(self.reg_c);
        self.pointer += 2;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_module_parsing() {
        let state = crate::ProgramState::parse(
            "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0",
        );
        assert_eq!(state.reg_b, 0);
        assert_eq!(state.program, vec![0, 1, 5, 4, 3, 0]);
    }

    #[test]
    fn test_bst() {
        let mut state = crate::ProgramState::parse(
            "Register A: 0
Register B: 0
Register C: 9

Program: 2,6",
        );

        state.run();
        assert_eq!(state.reg_b, 1);
    }

    #[test]
    fn test_a_10() {
        let mut state = crate::ProgramState::parse(
            "Register A: 10
Register B: 0
Register C: 0

Program: 5,0,5,1,5,4",
        );

        state.run();
        assert_eq!(state.output, vec![0, 1, 2]);
    }

    #[test]
    fn test_a_2024() {
        let mut state = crate::ProgramState::parse(
            "Register A: 2024
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0",
        );

        state.run();
        assert_eq!(state.reg_a, 0);
        assert_eq!(state.output, vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
    }

    #[test]
    fn test_b_29() {
        let mut state = crate::ProgramState::parse(
            "Register A: 0
Register B: 29
Register C: 0

Program: 1,7",
        );

        state.run();
        assert_eq!(state.reg_b, 26);
    }

    #[test]
    fn test_b_2024() {
        let mut state = crate::ProgramState::parse(
            "Register A: 0
Register B: 2024
Register C: 43690

Program: 4,0",
        );

        state.run();
        assert_eq!(state.reg_b, 44354);
    }
}
