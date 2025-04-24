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

        let number_of_fences = find_number_of_fences(region, &grid);

        fence_cost += area * number_of_fences;
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
        assert_eq!(result, "80".to_string())
    }

    #[test]
    fn test_e_example() {
        let result = process(
            "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE",
        );
        assert_eq!(result, "236".to_string())
    }

    #[test]
    fn test_diagonal_example() {
        let result = process(
            "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA",
        );
        assert_eq!(result, "368".to_string())
    }

    #[test]
    fn test_large_example() {
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
        assert_eq!(result, "1206".to_string())
    }
}
