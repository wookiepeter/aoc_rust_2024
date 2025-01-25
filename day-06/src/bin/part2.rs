use std::collections::{HashMap, HashSet};

use aoc_util::{direction::Direction, grid::Grid};

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let grid = Grid::<char>::new_char_grid(input);

    // this is for normal walking of the map
    let start_position = *grid.find_positions(&'^').first().unwrap();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    visited.insert(start_position);
    let mut position = start_position;
    let mut direction = Direction::Up;

    // this is for finding loops
    let mut map: HashMap<Direction, Vec<(usize, usize)>> = HashMap::new();

    while let Some((new_pos, c)) = grid.get_direct_neighbor(position, direction) {
        match c {
            '#' => {
                direction = direction.cw();
            }
            _ => {
                visited.insert(new_pos);
                position = new_pos;
            }
        }
    }

    visited.len().to_string()
}

#[cfg(test)]
mod tests {
    use crate::process;

    #[test]
    fn test_example() {
        let result = process(
            "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...",
        );
        assert_eq!(result, "6".to_string())
    }
}
