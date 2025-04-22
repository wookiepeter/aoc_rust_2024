fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let mut grave_yard = input.to_string();
    for _ in 0..25 {
        grave_yard = blink_once(&grave_yard);
    }
    grave_yard.split(' ').count().to_string()
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
    use crate::blink_once;
    use crate::process;

    #[test]
    fn test_process() {
        let result = process("125 17");
        assert_eq!(result, "55312");
    }

    #[test]
    fn test_blink() {
        let result = blink_once("0 1 10 99 999");
        assert_eq!(result, "1 2024 1 0 9 9 2021976".to_string())
    }

    #[test]
    fn test_multiple_blinks() {
        let input = "125 17";

        let output = blink_once(input);
        assert_eq!(output, "253000 1 7");

        let output = blink_once(&output);
        assert_eq!(output, "253 0 2024 14168");

        let output = blink_once(&output);
        assert_eq!(output, "512072 1 20 24 28676032");

        let output = blink_once(&output);
        assert_eq!(output, "512 72 2024 2 0 2 4 2867 6032");
        let output = blink_once(&output);

        assert_eq!(output, "1036288 7 2 20 24 4048 1 4048 8096 28 67 60 32");
        let output = blink_once(&output);

        assert_eq!(
            output,
            "2097446912 14168 4048 2 0 2 4 40 48 2024 40 48 80 96 2 8 6 7 6 0 3 2"
        );
    }
}
