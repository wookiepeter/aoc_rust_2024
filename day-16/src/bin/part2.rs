use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

use aoc_util::{
    direction::Direction,
    grid::{self, Grid},
    grid_display,
};
use day_16::SearchState;

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

// Part 1 -> find the score of the ideal path
// Part 2 -> find all the tiles of the ideal path (i.e. number of tiles)
// Start at end point
// check all potential neighbors
// if they have a lower score, then the previous node -> they are on the ideal path ->
// add to hashset and add their new neighbors to the queue to check

fn process(input: &str) -> String {
    let map = Grid::<char>::new_char_grid(input);

    let start_position = *map.find_positions(&'S').first().unwrap();
    let end_position = *map.find_positions(&'E').first().unwrap();

    // compute path scores
    let path_scores = compute_path_scores(&map, start_position);
    // Direction can't simply be removed
    // we have to keep track of direction both here and when walking backwards

    println!("{:?}", path_scores);
    let mut best_path_tiles: HashSet<(usize, usize)> = HashSet::new();

    let mut queue: Vec<((usize, usize), Direction)> = Vec::new();
    // queue.push((end_position, Direction::Right));
    queue.push((end_position, Direction::Up));

    while let Some((position, direction)) = queue.pop() {
        let score = *path_scores.get(&(position, direction)).unwrap();
        println!("looking at neighbors for {:?} with score {score}", position);
        for (neighbor_pos, dir) in filter_valid_neighbors(position, &map) {
            if let Some(neighbor_score) = path_scores.get(&(neighbor_pos, dir.opposite())) {
                println!(
                    "Found neighbor at {:?} with score {neighbor_score}",
                    neighbor_pos
                );
                // can't check score blankly -> have to check score in the same direction or find smallest neighbor -> do both
                if *neighbor_score < score && !best_path_tiles.contains(&neighbor_pos) {
                    best_path_tiles.insert(neighbor_pos);
                    queue.push((neighbor_pos, dir));
                }
            }
        }
        // find all valid neighbors -> if their scores are lower than yours -> add them to result and put them in the queue
    }

    let mut display = grid_display::GridDisplay::from_grid(&map);
    display.apply_char_layer('O', best_path_tiles.iter().cloned());
    println!("{}", display.to_string());

    best_path_tiles.len().to_string()
}

fn filter_valid_neighbors(
    position: (usize, usize),
    grid: &Grid<char>,
) -> Vec<((usize, usize), Direction)> {
    Direction::CLOCKWISE
        .iter()
        .filter_map(|dir| match grid.get_direct_neighbor(position, *dir) {
            Some((_, '#')) => None,
            Some((neighbor_pos, _)) => Some((neighbor_pos, *dir)),
            _ => None,
        })
        .collect()
}

fn compute_path_scores(
    map: &Grid<char>,
    start_position: (usize, usize),
) -> HashMap<((usize, usize), Direction), usize> {
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

    visited
}

#[cfg(test)]
mod tests {
    use crate::process;

    #[test]
    fn test_example_1() {
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
        assert_eq!(result, "45".to_string())
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
        assert_eq!(result, "64".to_string())
    }
}
