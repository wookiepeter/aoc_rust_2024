use aoc_util::{direction::Direction, grid::Grid};
use day_15::*;
fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    // parse input
    let (mut map, directions) = parse_input(input);
    let robot = *map.find_positions(&'@').first().unwrap();

    // move robot
    process_robot_moves(&mut map, directions, robot);

    let result: usize = map
        .find_positions(&'O')
        .iter()
        .map(|pos| 100 * pos.1 + pos.0)
        .sum();

    result.to_string()
}

fn process_robot_moves(
    map: &mut Grid<char>,
    directions: Vec<Direction>,
    mut robot: (usize, usize),
) {
    for direction in directions {
        // check ahead of robot in direction
        // if it's a # -> continue
        // if it's a . -> move robot, replace old robot with . and continue
        // if it's a O -> check all the spaces ahead until you hit something other than O
        //          -> if it's a . -> make it an o and then move robot
        //          -> else continue

        match map.get_direct_neighbor(robot, direction).unwrap() {
            (_, '#') => continue,
            (_, '.') => robot = move_robot(robot, direction, map),
            (_, 'O') => {
                let mut cur_pos = robot;
                while let Some((neighbor, c)) = map.get_direct_neighbor(cur_pos, direction) {
                    cur_pos = neighbor;
                    match c {
                        // look for the first dot
                        'O' => continue,
                        '.' => {
                            map.set(neighbor, 'O');
                            robot = move_robot(robot, direction, map);
                            break;
                        }
                        _ => break,
                    }
                }
            }
            _ => panic!("Invalid character in map"),
        }
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
    fn test_short_example() {
        let result = process(
            "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<",
        );
        assert_eq!(result, "2028".to_string())
    }

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
        assert_eq!(result, "10092".to_string())
    }
}
