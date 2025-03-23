use std::collections::{HashMap, HashSet};

use itertools::Itertools;

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let grid = aoc_util::grid::Grid::<char>::new_char_grid(input);
    let map_size = (grid.size.0 as i32, grid.size.1 as i32);

    let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    for (y, row) in grid.array.iter().enumerate() {
        for (x, c) in row.iter().enumerate().filter(|(_, c)| **c != '.') {
            let position = (x as i32, y as i32);
            match antennas.get_mut(c) {
                Some(vec) => vec.push(position),
                None => {
                    antennas.insert(*c, vec![position]);
                }
            }
        }
    }

    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();

    for (_, positions) in antennas.iter() {
        if positions.len() > 1 {
            positions.iter().for_each(|pos| {
                antinodes.insert(*pos);
            });
        }
        positions.iter().combinations(2).for_each(|arr| {
            let (first, second) = (arr[0], arr[1]);
            let dist = (second.0 - first.0, second.1 - first.1);
            let mut first_antinode = (second.0 + dist.0, second.1 + dist.1);
            let mut second_antinode = (first.0 - dist.0, first.1 - dist.1);

            // repeatedly move antinode by dist until you leave map
            while first_antinode._is_in_map(map_size) {
                antinodes.insert(first_antinode);
                first_antinode = (first_antinode.0 + dist.0, first_antinode.1 + dist.1);
            }
            while second_antinode._is_in_map(map_size) {
                antinodes.insert(second_antinode);
                second_antinode = (second_antinode.0 - dist.0, second_antinode.1 - dist.1);
            }
        });
    }

    antinodes.len().to_string()
}

trait InMap {
    type Item;
    fn _is_in_map(&self, map_size: (i32, i32)) -> bool;
}

impl InMap for (i32, i32) {
    type Item = (i32, i32);

    fn _is_in_map(&self, map_size: (i32, i32)) -> bool {
        0 <= self.0 && self.0 < map_size.0 && 0 <= self.1 && self.1 < map_size.1
    }
}

#[cfg(test)]
mod tests {
    use crate::process;

    #[test]
    fn test_example() {
        let result = process(
            "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............",
        );
        assert_eq!(result, "34".to_string())
    }

    #[test]
    fn test_basic_example() {
        let result = process(
            "T.........
...T......
.T........
..........
..........
..........
..........
..........
..........
..........",
        );
        assert_eq!(result, "9".to_string())
    }
}
