use std::ops::Sub;

use aoc_util::grid::*;

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let grid: Grid<char> = Grid::<char>::new_char_grid(input);
    let directions: Vec<(i32, i32)> = vec![
        (-1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
    ];

    let mut result: u32 = 0;

    for y in 0..grid.size.1 {
        for x in 0..grid.size.0 {
            if grid.get((x, y)).unwrap().eq(&'X') {
                // check in each direction if the contents of mas are still available for the next 3 iterations
                // gotta figure out how to use the positions here,
                // maybe we actually need A Vector class that can Convert to (usize, usize)?!
                for dir in &directions {
                    if test_direction((x, y), dir, &grid) {
                        result += 1;
                    }
                }
            }
        }
    }

    result.to_string()
}

fn test_direction(start: (usize, usize), dir: &(i32, i32), grid: &Grid<char>) -> bool {
    let xmas: Vec<char> = vec!['X', 'M', 'A', 'S'];

    let end_x = (start.0 as i32) + dir.0 * 3;
    let end_y = (start.1 as i32) + dir.1 * 3;
    if end_x < 0 || end_x >= grid.size.0 as i32 || end_y < 0 || end_y >= grid.size.1 as i32 {
        return false;
    }

    for i in 1..=3 {
        let target: (usize, usize) = (
            (start.0 as i32 + dir.0 * i) as usize,
            (start.1 as i32 + dir.1 * i) as usize,
        );
        if grid.get(target) != Some(&xmas[i as usize]) {
            return false;
        }
    }

    true
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
        assert_eq!(result, "18".to_string())
    }
}
