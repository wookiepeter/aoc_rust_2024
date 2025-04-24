use aoc_util::grid::Grid;
use day_12::*;

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
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
