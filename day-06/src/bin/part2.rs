use std::{collections::HashSet, iter};

use aoc_util::{direction::Direction, grid::Grid};

// After looking at other solutions -> this is was way too overengineered and had a logic flaw
// i found the flaw and simplified a bit, but more could be done
// A lot of easier (and still fast) solutions just let the guard walk around to test loops and use
// a hashmap for duplicate moves
// see this example here: https://github.com/jstnd/programming-puzzles/blob/master/rust/src/aoc/year2024/day06.rs
// Rust is a very fast language, sometimes the easiest thing to do is just to use the primitive
// implementation which should have avoided a shitton of issues here
//
// BUT IT WORKS, and it's actually not that slow
fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let grid = Grid::<char>::new_char_grid(input);
    let start_position = *grid.find_positions(&'^').first().unwrap();

    let obstruction_map = ObstructionMap::from_grid(&grid);

    let mut current_ray = Some((start_position, Direction::Up));
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    // Initial loop that collects the default path, i.e. all the places where we could potentially place obstacles
    while let Some((turn_point, dir)) = current_ray {
        current_ray = match obstruction_map.find_turn_point(&turn_point, &dir) {
            // turns in place -> no possible obstacles
            Some(next_turn_point) if next_turn_point == turn_point => {
                Some((next_turn_point, dir.cw()))
            }
            // normal case -> line points
            Some(next_turn_point) => {
                // order should not matter -> build range over valid points -> turn into iterator
                let (line_start, _) = obstruction_map
                    .grid
                    .get_direct_neighbor(turn_point, dir)
                    .expect("Line should have an endpoint");
                ObstructionMap::give_line_points(line_start, next_turn_point, dir)
                    .iter()
                    .for_each(|position| {
                        visited.insert(*position);
                    });
                Some((next_turn_point, dir.cw()))
            }
            // check everything up to the edge of the grid -> then yield None to stop the loop
            None => {
                let (line_start, _) = obstruction_map
                    .grid
                    .get_direct_neighbor(turn_point, dir)
                    .expect("Line should have an endpoint");
                obstruction_map
                    .give_ray_points(line_start, dir)
                    .iter()
                    .for_each(|position| {
                        visited.insert(*position);
                    });

                None
            }
        }
    }

    let mut obstacle_counter: usize = 0;

    // put an obstacle on each position of the path and see if the new map loops
    for obstacle in visited {
        let mut modified_grid = grid.clone();
        modified_grid.array[obstacle.1][obstacle.0] = '#';
        let obstruction_map = ObstructionMap::from_grid(&modified_grid);
        if obstruction_map.is_loop(start_position, Direction::Up) {
            obstacle_counter += 1;
        }
    }

    obstacle_counter.to_string()
}

struct ObstructionMap<'a> {
    // store a ref to grid -> need access to it's utility functions
    grid: &'a Grid<char>,
    directional_obstacles: Vec<Vec<Vec<usize>>>,
}

// builds a map with all the turn points sorted by directions
impl<'a> ObstructionMap<'a> {
    pub fn from_grid(grid: &'a Grid<char>) -> Self {
        let mut directional_obstacles: Vec<Vec<Vec<usize>>> = vec![vec![]; 4];
        // Initialize all directions the array for all columns / rows with the correct size based on grid.size
        Direction::CLOCKWISE
            .iter()
            .enumerate()
            .for_each(|(index, dir)| {
                // let length = pick_value(point, dir)
                directional_obstacles[index] = vec![vec![]; Self::pick_column_row(&grid.size, dir)];
            });

        // go through all obstructions in the field
        grid.find_positions(&'#').iter().for_each(|obstruction| {
            // grab neighbors, filter anything out of maps
            Direction::CLOCKWISE
                .iter()
                .filter_map(|dir| {
                    grid.get_direct_neighbor(*obstruction, (*dir).opposite())
                        .map(|(neighbor, _)| (dir, neighbor))
                })
                .for_each(|(dir, turn_point)| {
                    let row_column = Self::pick_column_row(&turn_point, dir);
                    let value = Self::pick_index(&turn_point, dir);
                    directional_obstacles[*dir as usize][row_column].push(value);
                });
        });

        // simply sort everything, directional finding should be done based on Direction -> KISS
        for directional_lists in &mut directional_obstacles {
            for list in directional_lists {
                list.sort();
            }
        }
        ObstructionMap {
            grid,
            directional_obstacles,
        }
    }

    pub fn is_loop(&self, start_pos: (usize, usize), initial_direction: Direction) -> bool {
        // first ray sent out needs to be compared
        let initial_ray = Some((start_pos, initial_direction));
        let mut next_ray = initial_ray;
        let mut loop_detection: HashSet<((usize, usize), Direction)> = HashSet::new();

        while let Some((turn_point, dir)) = next_ray {
            // find the turn point ahead or equal to us
            let next_turn_point = self.find_turn_point(&turn_point, &dir);
            // map on option -> if no turn point loop should exit on next iteration
            next_ray = next_turn_point.map(|point| (point, dir.cw()));
            if next_ray == initial_ray {
                return true;
            }
            if next_ray.is_some() && !loop_detection.insert(next_ray.unwrap()) {
                // found an infinite loop -> return false
                return true;
            }
        }
        false
    }

    fn _obstacle_into_turnpoint(
        &self,
        obstacle: &(usize, usize),
        dir: &Direction,
    ) -> (usize, usize) {
        let (new_turn_point, _) = self
            .grid
            .get_direct_neighbor(*obstacle, dir.opposite())
            .expect("obstacle should have valid turnpoint, due to surrounding logic!");

        new_turn_point
    }

    // the following methods should ideally be utility functions on a proper Usize_point type!

    fn _obstacle_on_path(
        &self,
        obstacle: &(usize, usize),
        start: &(usize, usize),
        end: &(usize, usize),
        dir: &Direction,
    ) -> bool {
        let start_index = ObstructionMap::pick_index(start, dir);
        let end_index = ObstructionMap::pick_index(end, dir);
        let obstacle_index = ObstructionMap::pick_index(obstacle, dir);
        // Could possibly be an error source?!
        obstacle_index.is_within(start_index, end_index)
    }

    fn _obstacle_on_ray(
        &self,
        obstacle: &(usize, usize),
        start: &(usize, usize),
        dir: &Direction,
    ) -> bool {
        match dir {
            Direction::Up => start.1 > obstacle.1,
            Direction::Right => start.0 < obstacle.0,
            Direction::Down => start.1 < obstacle.1,
            Direction::Left => start.0 > obstacle.0,
        }
    }

    fn give_ray_points(&self, start: (usize, usize), dir: Direction) -> Vec<(usize, usize)> {
        let end_point = match dir {
            Direction::Up => (start.0, 0),
            Direction::Right => (self.grid.size.0 - 1, start.1),
            Direction::Down => (start.0, self.grid.size.1 - 1),
            Direction::Left => (0, start.1),
        };

        // this skips the step!
        Self::give_line_points(start, end_point, dir)
    }

    fn give_line_points(
        start: (usize, usize),
        end: (usize, usize),
        dir: Direction,
    ) -> Vec<(usize, usize)> {
        let start_index = ObstructionMap::pick_index(&start, &dir);
        let end_index = ObstructionMap::pick_index(&end, &dir);
        let row_column = ObstructionMap::pick_column_row(&start, &dir);
        // Range has to be created manually depending on direction since min and max fucks the last
        // element not being included in some cases
        let iter = match dir {
            Direction::Up | Direction::Left => end_index..(start_index + 1),
            Direction::Right | Direction::Down => start_index..(end_index + 1),
        };
        let result: Vec<(usize, usize)> = match dir {
            Direction::Up | Direction::Down => iter::repeat(row_column).zip(iter).collect(),
            Direction::Left | Direction::Right => iter.zip(iter::repeat(row_column)).collect(),
        };
        result
    }

    fn find_turn_point(&self, point: &(usize, usize), dir: &Direction) -> Option<(usize, usize)> {
        let mut iter = self.pick_lane(point, dir);
        match dir {
            Direction::Up => iter
                .rev()
                .find(|index| **index <= ObstructionMap::pick_index(point, dir))
                .map(|y| (point.0, *y)),
            Direction::Right => iter
                .find(|index| **index >= ObstructionMap::pick_index(point, dir))
                .map(|x| (*x, point.1)),
            Direction::Down => iter
                .find(|index| **index >= ObstructionMap::pick_index(point, dir))
                .map(|y| (point.0, *y)),
            Direction::Left => iter
                .rev()
                .find(|index| **index <= ObstructionMap::pick_index(point, dir))
                .map(|x| (*x, point.1)),
        }
    }

    fn pick_lane(&self, point: &(usize, usize), dir: &Direction) -> std::slice::Iter<'_, usize> {
        self.directional_obstacles[*dir as usize][ObstructionMap::<'a>::pick_column_row(point, dir)]
            .iter()
    }

    /// pick the index in the column or row of a point, based on the Direction;
    /// does not reverse order -> same index for left and right
    fn pick_index(point: &(usize, usize), dir: &Direction) -> usize {
        match dir {
            Direction::Up | Direction::Down => point.1,
            Direction::Left | Direction::Right => point.0,
        }
    }

    /// picks the column or row of a point indicated by the direction
    fn pick_column_row(point: &(usize, usize), dir: &Direction) -> usize {
        match dir {
            Direction::Up | Direction::Down => point.0,
            Direction::Left | Direction::Right => point.1,
        }
    }
}

pub trait Within {
    type Item;
    fn is_within(&self, a: Self::Item, b: Self::Item) -> bool;
}

impl Within for usize {
    type Item = usize;

    fn is_within(&self, a: Self::Item, b: Self::Item) -> bool {
        (a <= *self && *self <= b) || (b <= *self && *self <= a)
    }
}

#[cfg(test)]
mod tests {
    use crate::{process, Within};

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

    #[test]
    fn test_obstacle_at_turnpoint() {
        let result = process(
            ".#...
.....
....#
#^...
...#.",
        );
        assert_eq!(result, "2".to_string());
    }

    #[test]
    fn test_180_example() {
        let result = process(
            ".....
..#..
...#.
#.^..
..#..",
        );
        assert_eq!(result, "2".to_string())
    }

    /*
    TODO: In this case the O is positioned on a path that has already been used by the guard and
    therefore this block should be invalid
    -> solve this then continue looking for more invalid cases
    ...........#.....#......
    ...........R......D#....
    ...#.....##U......S.....
    ..................O...#.
    ..................#.....
    ..#.....................
    ....................#...
    ........................
    .#........^.............
    ..........#..........#..
    ..#.....#..........#....
    ........#.....#..#......

         */
    #[test]
    fn test_reddit_example() {
        let result = process(
            "...........#.....#......
...................#....
...#.....##.............
......................#.
..................#.....
..#.....................
....................#...
........................
.#........^.............
..........#..........#..
..#.....#..........#....
........#.....#..#......",
        );
        assert_eq!(result, "19".to_string())
    }

    #[test]
    fn test_is_within() {
        assert!(2usize.is_within(1, 3));
        assert!(0usize.is_within(0, 5));
        assert!(!6usize.is_within(0, 5));
    }
}

#[cfg(test)]
mod obstruction_map_tests {
    use aoc_util::grid;

    use crate::Direction;
    use crate::ObstructionMap;

    #[test]
    fn test_pick_row() {
        assert_eq!(
            ObstructionMap::pick_column_row(&(2, 3), &Direction::Left),
            3
        );
        assert_eq!(ObstructionMap::pick_column_row(&(5, 0), &Direction::Up), 5);
    }

    #[test]
    fn test_pick_index() {
        assert_eq!(ObstructionMap::pick_index(&(4, 3), &Direction::Right), 4);
        assert_eq!(ObstructionMap::pick_index(&(4, 3), &Direction::Left), 4);
    }

    #[test]
    fn build_super_basic_map() {
        let map_string = "...
#..
..#";
        let char_grid = grid::Grid::<char>::new_char_grid(map_string);
        let obstruct_grid = ObstructionMap::from_grid(&char_grid);

        let manual_map = vec![
            vec![vec![2], vec![], vec![]],
            vec![vec![], vec![], vec![1]],
            vec![vec![0], vec![], vec![1]],
            vec![vec![], vec![1], vec![]],
        ];

        assert_eq!(obstruct_grid.directional_obstacles, manual_map);
    }

    #[test]
    fn test_find_turn_point() {
        let map_string: &str = "#....
....#
.#...
#....
...#.";

        let char_grid = grid::Grid::<char>::new_char_grid(map_string);
        let obstruct_grid = ObstructionMap::from_grid(&char_grid);

        assert_eq!(
            obstruct_grid.find_turn_point(&(0, 1), &Direction::Right),
            Some((3, 1))
        );
        assert_eq!(
            obstruct_grid.find_turn_point(&(3, 1), &Direction::Down),
            Some((3, 3))
        );
        assert_eq!(
            obstruct_grid.find_turn_point(&(3, 3), &Direction::Left),
            Some((1, 3))
        );
        assert_eq!(
            obstruct_grid.find_turn_point(&(4, 4), &Direction::Up),
            Some((4, 2))
        );
        // immediate turn
        assert_eq!(
            obstruct_grid.find_turn_point(&(1, 3), &Direction::Up),
            Some((1, 3))
        );
    }

    #[test]
    fn test_line_points() {
        let vec = ObstructionMap::give_line_points((3, 2), (3, 0), Direction::Up);
        assert!(vec.len() == 3);
        assert!(vec.contains(&(3, 1)));
        assert!(vec.contains(&(3, 0)));
        assert!(vec.contains(&(3, 2)));
    }
}
