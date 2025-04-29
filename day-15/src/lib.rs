use aoc_util::direction::Direction;
use aoc_util::grid::Grid;

pub fn parse_input(input: &str) -> (Grid<char>, Vec<Direction>) {
    let (map, directions) = input.split_once("\n\n").unwrap();
    (
        Grid::<char>::new_char_grid(map),
        parse_directions(directions),
    )
}

fn parse_directions(input: &str) -> Vec<Direction> {
    input
        .lines()
        .flat_map(|line| {
            line.chars().map(|c| match c {
                '^' => Direction::Up,
                '>' => Direction::Right,
                'v' => Direction::Down,
                '<' => Direction::Left,
                invalid => panic!("Invalid char {}!", invalid),
            })
        })
        .collect()
}
