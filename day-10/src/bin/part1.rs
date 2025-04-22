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
        .map(|trail_head| find_trails_from_head(&grid, trail_head))
        .sum();

    result.to_string()
}

fn find_trails_from_head(grid: &Grid<i32>, trail_head: &(usize, usize)) -> usize {
    let mut current_trails = vec![*trail_head];
    for i in 1..=9 {
        current_trails = current_trails
            .iter()
            .flat_map(|pos| grid.get_neighbors(*pos).into_iter())
            .filter(|neighbor| grid.get(*neighbor) == Some(&i))
            .collect();
        if current_trails.is_empty() {
            break;
        }
    }
    let trail_ends: HashSet<(usize, usize)> = HashSet::from_iter(current_trails);
    trail_ends.len()
}

#[cfg(test)]
mod tests {
    use crate::process;

    #[test]
    fn test_2_trails() {
        let result = process(
            "...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9",
        );
        assert_eq!(result, "2".to_string())
    }

    #[test]
    fn test_2_trailheads() {
        let result = process(
            "10..9..
2...8..
3...7..
4567654
...8..3
...9..2
.....01",
        );
        assert_eq!(result, "3".to_string())
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
        assert_eq!(result, "36".to_string())
    }
}
