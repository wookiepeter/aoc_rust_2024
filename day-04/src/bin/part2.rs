use aoc_util::grid::Grid;

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let grid: Grid<char> = Grid::<char>::new_char_grid(input);

    let corner_combinations: Vec<Vec<char>> = vec![
        vec!['M', 'M', 'S', 'S'],
        vec!['S', 'M', 'M', 'S'],
        vec!['S', 'S', 'M', 'M'],
        vec!['M', 'S', 'S', 'M'],
    ];

    let mut result = 0;

    for y in 1..(grid.size.1 - 1) {
        for x in 1..(grid.size.0 - 1) {
            if grid.get((x, y)) == Some(&'A') {
                let neighbors = vec![
                    *grid.get((x - 1, y - 1)).unwrap(),
                    *grid.get((x + 1, y - 1)).unwrap(),
                    *grid.get((x + 1, y + 1)).unwrap(),
                    *grid.get((x - 1, y + 1)).unwrap(),
                ];

                if corner_combinations.contains(&neighbors) {
                    result += 1;
                }
            }
        }
    }

    result.to_string()
}

#[cfg(test)]
mod tests {
    use crate::process;

    #[test]
    fn test_example() {
        let result = process(
            "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX",
        );
        assert_eq!(result, "9".to_string())
    }
}
