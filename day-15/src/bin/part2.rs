use std::collections::HashSet;

use aoc_util::{direction::Direction, grid::Grid, grid_display::GridDisplay};

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let mut modified_input = input.replace('.', "..");
    modified_input = modified_input.replace('#', "##");
    modified_input = modified_input.replace('O', "[]");
    modified_input = modified_input.replace('@', "@.");

    let (mut map, directions) = day_15::parse_input(&modified_input);
    let mut robot = *map.find_positions(&'@').first().unwrap();

    for direction in directions {
        // for # and . nothing changes
        // for
        match map.get_direct_neighbor(robot, direction).unwrap() {
            (_, '#') => continue,
            (_, '.') => robot = move_robot(robot, direction, &mut map),
            (_, '[') | (_, ']') => {
                // probably best to write a separate function here?!
                robot = move_boxes_with_robot(robot, direction, &mut map)
            }
            (pos, c) => panic!("Invalid char {} at position {:?}", c, pos),
        }
    }

    let result: usize = map
        .find_positions(&'[')
        .iter()
        .map(|pos| 100 * pos.1 + pos.0)
        .sum();

    result.to_string()
}

fn move_boxes_with_robot(
    robot: (usize, usize),
    direction: Direction,
    map: &mut Grid<char>,
) -> (usize, usize) {
    // find all boxes that should be moved -> empty if no boxes should be moved
    // split into vertical / horizontal
    // maybe split into separate function
    let boxes_to_move = find_boxes_to_move(robot, direction, map);
    // no boxes to move detected -> don't move the robot
    if boxes_to_move.is_none() {
        return robot;
    }
    let boxes_to_move = boxes_to_move.unwrap();

    if boxes_to_move.is_empty() {
        return robot;
    }
    // move all boxes separately -> start with the row furthest from the robot so nothing get's overwritten
    for old_position in boxes_to_move.iter().rev() {
        let new_position = map.get_direct_neighbor(*old_position, direction).unwrap().0;
        map.set(new_position, *map.get(*old_position).unwrap());
        map.set(*old_position, '.');
    }

    // move robot
    move_robot(robot, direction, map)
}

/// detects a box and returns both it's positions []
/// ordered -> row / column closest to the first
fn find_boxes_to_move(
    robot: (usize, usize),
    direction: Direction,
    map: &mut Grid<char>,
) -> Option<Vec<(usize, usize)>> {
    match direction {
        Direction::Left | Direction::Right => {
            let mut result = Vec::new();
            let mut current_pos = robot;

            while let Some((next_pos, c)) = map.get_direct_neighbor(current_pos, direction) {
                match *c {
                    '#' => return None,
                    '.' => return Some(result),
                    _ => {
                        result.push(next_pos);
                        current_pos = next_pos;
                    }
                }
            }

            panic!("Shouldn't reach this, map state is probably invalid!");
        }
        _ => {
            let mut result = vec![];
            let mut current_row = HashSet::new();
            current_row.insert(robot);
            loop {
                // go through the last row
                // grab all the boxes the positions that would be moved
                let last_row: Vec<(usize, usize)> = current_row.iter().cloned().collect();
                current_row.clear();
                for pos in last_row {
                    // if any of them are '#' i.e. -> can't move -> return None
                    match map.get_direct_neighbor(pos, direction).unwrap() {
                        (_, '#') => {
                            return None;
                        }
                        (pos, '[') | (pos, ']') => {
                            current_row.insert(pos);
                            current_row.insert(get_other_box_position(pos, map));
                        }
                        _ => continue,
                    }
                }

                // check if all boxes are '.' i.e. -> current_row.empty() -> found all the boxes you should move
                if current_row.is_empty() {
                    return Some(result);
                }
                // else some of them are boxes -> add boxes to result and continue
                result.extend(current_row.iter().cloned());
            }
        }
    }
}

fn get_other_box_position(position: (usize, usize), map: &Grid<char>) -> (usize, usize) {
    match *map.get(position).unwrap() {
        '[' => (position.0 + 1, position.1),
        ']' => (position.0 - 1, position.1),
        _ => panic!("no box on selected position"),
    }
}

/// Moves the robot on the grid, returns it's new position and replaces it's old with a '.'
/// should only be used if you're confident a move is actually valid
fn move_robot(robot: (usize, usize), direction: Direction, map: &mut Grid<char>) -> (usize, usize) {
    let (new_robot, _) = map.get_direct_neighbor(robot, direction).unwrap();
    map.set(new_robot, '@');
    // set old position
    map.set(robot, '.');
    new_robot
}

#[cfg(test)]
mod tests {
    use crate::process;
    #[test]
    fn test_large_example() {
        let result = process(
            "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^",
        );
        assert_eq!(result, "9021".to_string())
    }
}
