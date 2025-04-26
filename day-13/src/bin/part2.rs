use day_13::Machine;

// Had to turn down the tolerance way down for this one but it still works!!! :)
const TOLERANCE: f64 = 1e-2;
const PRIZE_OFFSET: f64 = 10000000000000.0;

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

/*
BETTER SOLUTION
while my approach of solving a system of linear equations sorta works there is still a much better solution.

A machine in this game essentially represents a system of 2 equations  with 2 unknown variables
#1: prize.0 = a.0 * a + b.0 * b
#2: prize.1 = a.1 * a + b.1 * b

Because it is a system of equations we can solve that system.
The easiest approach do code that is probably solving by substraction

This allows you to arrange the equations in such a way that you can eliminate one of the variables
a_value = (prize.0 * b.1 - prize.1 * b.0) / (a.0 * b.1 - a.1 * b.0)
This gives you a concrete value for a which you can than put into one of the original equations
prize.0 = a.0 * a_value + b.0 * b
and then solve that equation for b to get the second value
b = (prize.0 - a.0 * a_value) / b.0

Now all that is left is to check if that solution actually fits (even numbers, ...)

*/
fn process(input: &str) -> String {
    let machines = day_13::parse_machines(input);
    let mut result = 0u64;

    // Limit for each button is 100
    // Potential ideas:
    // reduce the possible solutions as much as possible (maximum button presses for each button)
    // use the relation between both numbers to determine which i have to use how often
    // i.e. 67 / 22 can only fit so often inside 8400 / 5400 ->
    // if the relation of the target position is not between the 2 numbers it's not possible

    for Machine {
        button_a,
        button_b,
        prize,
    } in machines
    {
        let button_a = (button_a.0 as f64, button_a.1 as f64);
        let button_b = (button_b.0 as f64, button_b.1 as f64);
        let prize = (prize.0 as f64 + PRIZE_OFFSET, prize.1 as f64 + PRIZE_OFFSET);
        let gradient_a = button_a.1 / button_a.0;
        let gradient_b = button_b.1 / button_b.0;
        let gradient_prize = prize.1 / prize.0;

        println!("Testing button with prize: {:?}", prize);

        if (gradient_a - gradient_b).abs() < TOLERANCE {
            // TODO: Handle special case where both gradients are aligned and there needs to be
            // a cost comparison
            // My input didn't hit that case so we just panic and run :)
            panic!("I AM SPECIAL");
        }

        // Can probably be skipped since the remainder of this solution should be super efficient
        // gradient of prize not between gradient a or b -> impossible to win
        if !((gradient_prize <= gradient_a && gradient_prize >= gradient_b)
            || (gradient_prize <= gradient_b && gradient_prize >= gradient_a))
        {
            continue;
        }

        // translation along y-axis for line a (i.e. y = gradient_a * x) so that it goes through the point prize
        let translate_a = prize.1 - (prize.0 * gradient_a);
        // find point of intersection for line a' (with translation) and line b
        // gradient_b * x = gradient_a * x + translate_a
        // -> (gradient_b - gradient_a) * x = translate_a
        // -> x = translate_a / (gradient_b - gradient_a)
        let intersect_x = translate_a / (gradient_b - gradient_a);
        let intersect_y = gradient_b * intersect_x;

        let button_b_presses = intersect_x / button_b.0;
        // test if button B presses is round and if button presses equal for both components
        if !((button_b_presses.round() - button_b_presses).abs() < TOLERANCE
            && (button_b_presses - (intersect_y / button_b.1).abs() < TOLERANCE))
        {
            continue;
        }

        // check the same for the remaining vec from the intersect point to the prize
        let remaining_vec_x = prize.0 - intersect_x;
        let remaining_vec_y = prize.1 - intersect_y;

        // let button_a_presses = remaining_vec_x / button_a.0;
        let button_a_presses = remaining_vec_x / button_a.0;
        // test if button A presses is round and if button presses equal for both components
        if !((button_a_presses.round() - button_a_presses).abs() < TOLERANCE
            && (button_a_presses - (remaining_vec_y / button_a.1).abs() < TOLERANCE))
        {
            continue;
        }

        result += button_a_presses.round() as u64 * 3 + button_b_presses.round() as u64;

        // TODO: !!! ADD SPECIAL CASE FOR WHEN BOTH ARE ALIGNED
        // have to check stepsize and also find the cheapest way because A is more expensive^^
    }

    result.to_string()
}

#[cfg(test)]
mod tests {
    use crate::process;

    #[test]
    fn test_example() {
        let result = process(
            "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279",
        );
        assert_eq!(result, "480".to_string())
    }
}
