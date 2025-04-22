use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let mut memory = HashMap::new();
    process_partial(input, 75, &mut memory).to_string()
}

// processes a partial section of the graveyard for a set amount of ticks
// this is mainly so i can use Memoization to reuse already known results
// And it works like a charm :)
fn process_partial(
    input: &str,
    remaining_blinks: usize,
    memory: &mut HashMap<(String, usize), usize>,
) -> usize {
    if remaining_blinks == 0 {
        return 1;
    }
    if let Some(final_grave_stones) = memory.get(&(input.to_string(), remaining_blinks)) {
        return *final_grave_stones;
    }

    blink_once(input)
        .split(' ')
        .map(|stone| {
            let result = process_partial(stone, remaining_blinks - 1, memory);
            memory.insert((stone.to_string(), remaining_blinks - 1), result);
            result
        })
        .sum()
}

fn blink_once(input: &str) -> String {
    let string_vec: Vec<String> = input
        .split(' ')
        .map(|grave_stone| {
            match grave_stone {
                // If the stone is engraved with the number 0, it is replaced by a stone engraved with the number 1.
                "0" => "1".to_string(),
                // If the stone is engraved with a number that has an even number of digits, it is replaced
                // by two stones. The left half of the digits are engraved on the new left stone, and the
                // right half of the digits are engraved on the new right stone. (The new numbers don't
                // keep extra leading zeroes: 1000 would become stones 10 and 0.)
                stone if stone.len() % 2 == 0 => {
                    let (left, right) = stone.split_at(stone.len() / 2);
                    let right = right
                        .parse::<u128>()
                        .unwrap_or_else(|_| {
                            panic!(
                                "{right} on split stone {grave_stone} couldn't be parsed correctly"
                            )
                        })
                        .to_string();
                    format!("{left} {right}")
                }
                // If none of the other rules apply, the stone is replaced by a new stone; the old stone's
                // number multiplied by 2024 is engraved on the new stone.
                stone => (stone.parse::<u128>().unwrap_or_else(|_| {
                    panic!("{stone} on leftover stone {grave_stone} couldn't be parsed correctly")
                }) * 2024)
                    .to_string(),
            }
        })
        .collect();

    string_vec.join(" ")
}

#[cfg(test)]
mod tests {
    use crate::process_partial;
    use std::collections::HashMap;

    #[test]
    fn test_process_partial() {
        let mut memory = HashMap::new();
        let result = process_partial("125 17", 25, &mut memory).to_string();
        assert_eq!(result, "55312");
    }
}
