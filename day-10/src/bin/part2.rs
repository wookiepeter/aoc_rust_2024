use std::collections::HashSet;

use aoc_util::grid::*;

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let grid: Grid<i32> = Grid::new(input, |c| {
        c.to_digit(10).map(|num| num as i32).unwrap_or(-1i32)
    });

    let result: usize = grid
        .find_positions(&0i32)
        .iter()
        .map(|trail_head| find_trail_rating(&grid, trail_head))
        .sum();

    result.to_string()
}

fn find_trail_rating(grid: &Grid<i32>, trail_head: &(usize, usize)) -> usize {
    let mut trails = vec![*trail_head];
    for i in 1..=9 {
        trails = trails
            .iter()
            .flat_map(|pos| grid.get_neighbors(*pos).into_iter())
            .filter(|neighbor| grid.get(*neighbor) == Some(&i))
            .collect();
        if trails.is_empty() {
            break;
        }
    }
    // let trail_ends: HashSet<(usize, usize)> = HashSet::from_iter(current_trails);
    trails.len()
}

#[cfg(test)]
mod tests {
    use crate::process;

    #[test]
    fn test_3_trails() {
        let result = process(
            ".....0.
..4321.
..5..2.
..6543.
..7..4.
..8765.
..9....",
        );
        assert_eq!(result, "3".to_string())
    }

    #[test]
    fn test_13_trails() {
        let result = process(
            "..90..9
...1.98
...2..7
6543456
765.987
876....
987....",
        );
        assert_eq!(result, "13".to_string())
    }

    #[test]
    fn test_example() {
        let result = process(
            "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732",
        );
        assert_eq!(result, "81".to_string())
    }
}
