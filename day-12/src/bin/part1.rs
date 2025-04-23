use std::collections::VecDeque;

use aoc_util::grid::*;
use rustc_hash::FxHashSet;

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

#[derive(Debug)]
struct Region {
    plant: char,
    plots: Vec<(usize, usize)>,
}

fn process(input: &str) -> String {
    let grid = Grid::<char>::new_char_grid(input);
    // build a of regions with all the regions
    let regions = find_regions(&grid);
    let mut fence_cost = 0;
    for region in regions {
        let area = region.plots.len();
        let perimeter = find_plot_perimeter(region, &grid);

        fence_cost += area * perimeter;
    }

    fence_cost.to_string()
}

fn find_plot_perimeter(region: Region, grid: &Grid<char>) -> usize {
    let mut perimeter = 0;

    for plot in region.plots {
        let neighbors = grid.get_neighbors(plot);
        // get_neighbors filters anything outside of the grid -> automatically part of the perimeter
        if neighbors.len() < 4 {
            perimeter += 4 - neighbors.len();
        }
        perimeter += neighbors
            .iter()
            .filter(|position| grid.get(**position).unwrap() != &region.plant)
            .count()
    }

    perimeter
}

fn find_regions(grid: &Grid<char>) -> Vec<Region> {
    let mut plots_in_regions: FxHashSet<(usize, usize)> = FxHashSet::default();
    let mut result = vec![];
    for position in grid.into_iter() {
        if !plots_in_regions.contains(&position) {
            let c = grid.get(position).unwrap();

            let region_plots = find_region(position, c, grid, &mut plots_in_regions);
            let region = Region {
                plant: *c,
                plots: region_plots,
            };
            result.push(region);
        }
    }

    result
}

fn find_region(
    position: (usize, usize),
    c: &char,
    grid: &Grid<char>,
    visited: &mut FxHashSet<(usize, usize)>,
) -> Vec<(usize, usize)> {
    let mut result = FxHashSet::default();
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    queue.push_front(position);
    while let Some(pos) = queue.pop_front() {
        if !visited.contains(&pos) && grid.get(pos).unwrap() == c {
            result.insert(pos);
            visited.insert(pos);
            grid.get_neighbors(pos)
                .iter()
                .filter(|neighbor| !visited.contains(neighbor))
                .for_each(|neighbor| queue.push_back(*neighbor));
        }
    }
    result.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use crate::process;

    #[test]
    fn test_example() {
        let result = process(
            "AAAA
BBCD
BBCC
EEEC",
        );
        assert_eq!(result, "140".to_string())
    }

    #[test]
    fn test_surround_example() {
        let result = process(
            "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO",
        );
        assert_eq!(result, "772".to_string())
    }

    #[test]
    fn test_long_example() {
        let result = process(
            "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE",
        );
        assert_eq!(result, "1930".to_string())
    }
}
