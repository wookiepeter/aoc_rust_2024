use std::collections::{BinaryHeap, HashMap, HashSet};

use aoc_util::{direction::Direction, grid::Grid, grid_display};
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

    let mut best_path_tiles: HashSet<(usize, usize)> = HashSet::new();
    best_path_tiles.insert(end_position);

    let mut queue: Vec<((usize, usize), Direction)> = Vec::new();
    // You have to select the start positions with the lowest score or both if score is equal
    let start_down = path_scores.get(&(end_position, Direction::Up));
    let start_right = path_scores.get(&(end_position, Direction::Right));

    if start_down.is_none() {
        queue.push((end_position, Direction::Right));
    } else if start_right.is_none() {
        queue.push((end_position, Direction::Up));
    } else {
        match start_down.cmp(&start_right) {
            std::cmp::Ordering::Less => queue.push((end_position, Direction::Up)),
            std::cmp::Ordering::Equal => {
                queue.push((end_position, Direction::Right));
                queue.push((end_position, Direction::Up));
            }
            std::cmp::Ordering::Greater => queue.push((end_position, Direction::Right)),
        }
    }

    while let Some(state) = queue.pop() {
        let score = *path_scores.get(&state).unwrap();
        for ((neighbor_pos, dir), score_change) in potential_prior_states(state, &map) {
            if let Some(neighbor_score) = path_scores.get(&(neighbor_pos, dir)) {
                // can't check score blankly -> have to check if score is correct for the specific move
                if let Some(computed_score) = score.checked_sub(score_change) {
                    if *neighbor_score == computed_score {
                        best_path_tiles.insert(neighbor_pos);
                        queue.push((neighbor_pos, dir));
                    }
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

#[allow(clippy::type_complexity)]
fn potential_prior_states(
    state: ((usize, usize), Direction),
    grid: &Grid<char>,
) -> Vec<(((usize, usize), Direction), usize)> {
    let (position, direction) = state;
    let mut result = vec![
        ((position, direction.cw()), 1000),
        ((position, direction.ccw()), 1000),
    ];
    match grid.get_direct_neighbor(position, direction.opposite()) {
        Some((previous_position, c)) if *c != '#' => {
            result.push(((previous_position, direction), 1));
        }
        _ => (),
    };
    result
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
