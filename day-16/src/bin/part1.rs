use day_16::SearchState;
use std::collections::{BinaryHeap, HashMap};

use aoc_util::{direction::Direction, grid::Grid};

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let map = Grid::<char>::new_char_grid(input);

    let start_position = *map.find_positions(&'S').first().unwrap();
    let end_position = *map.find_positions(&'E').first().unwrap();

    // note needs to be marked if not visited or updated if new score is lower than previous
    let mut visited: HashMap<((usize, usize), Direction), usize> = HashMap::new();
    let mut queue: BinaryHeap<SearchState> = BinaryHeap::new();
    let start_state = SearchState {
        position: start_position,
        direction: Direction::Right,
        score: 0,
    };
    visited.insert(start_state.without_score(), 0);
    queue.push(start_state);

    while let Some(state) = queue.pop() {
        // reached target
        if state.position == end_position {
            return state.score.to_string();
        }
        // check if state needs to be updated
        let previous_score = visited.get(&state.without_score());
        if previous_score.is_some() && *previous_score.unwrap() < state.score {
            continue;
        }
        visited.insert(state.without_score(), state.score);

        // add potential neighbors to queue
        // walk straight
        let neighbor = map
            .get_direct_neighbor(state.position, state.direction)
            .unwrap();
        if *neighbor.1 != '#' {
            let new_search_state = SearchState {
                position: neighbor.0,
                direction: state.direction,
                score: state.score + 1,
            };
            queue.push(new_search_state);
        }

        // turn left, turn right
        for dir in [state.direction.cw(), state.direction.ccw()] {
            let neighbor = map.get_direct_neighbor(state.position, dir).unwrap();
            if *neighbor.1 != '#' {
                let new_search_state = SearchState {
                    position: state.position,
                    direction: dir,
                    score: state.score + 1000,
                };
                queue.push(new_search_state);
            }
        }
    }

    panic!("Couldn't reach end of the maze!");
}

#[cfg(test)]
mod tests {
    use crate::process;

    #[test]
    fn test_example() {
        let result = process(
            "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############",
        );
        assert_eq!(result, "7036".to_string())
    }

    #[test]
    fn test_example_2() {
        let result = process(
            "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################",
        );
        assert_eq!(result, "11048".to_string())
    }
}
