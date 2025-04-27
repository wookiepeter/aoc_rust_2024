fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    process_with_size(input, (101, 103))
}

fn process_with_size(input: &str, bathroom_size: (i32, i32)) -> String {
    // parse robot settings
    let mut robots = day_14::parse_robots(input);
    let mut robots_by_quadrant = [0usize; 4];

    // simulate each robots position after 100 seconds
    robots
        .iter_mut()
        .for_each(|robot| robot.simulate_ticks(bathroom_size, 100));

    // collect robots per quadrant -> i.e. create a function that gives Option<usize>
    // and use that usize as an array index for the quadrant -> None on the center line
    robots
        .iter()
        .flat_map(|robot| robot.map_robot_to_quadrant(bathroom_size))
        .for_each(|quadrant| robots_by_quadrant[quadrant] += 1);

    let result = robots_by_quadrant
        .into_iter()
        .reduce(|acc, quadrant| acc * quadrant)
        .unwrap();

    result.to_string()
}

#[cfg(test)]
mod tests {
    use crate::process_with_size;

    #[test]
    fn test_example() {
        let result = process_with_size(
            "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3",
            (11, 7),
        );
        assert_eq!(result, "12".to_string())
    }
}
