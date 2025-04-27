use aoc_util::grid_display;
use day_14::*;
use std::io::{self};

/// This involved about 25 minutes of figuring out the pattern with manual search
/// Could / Should have been automated by loocking for cluster of robots.
fn main() -> io::Result<()> {
    let input = include_str!("./input.txt");
    let mut robots = parse_robots(input);

    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut tick = 0;
    loop {
        // figured out the interval where a lot of the robots where aligned by manually looking
        // through the first couple 100 ticks
        // then found the image by looking through the remaining position sets
        if tick % 101 == 99 {
            println!("{}", display_robots(&robots));
            println!("Tick {tick}");
            stdin.read_line(&mut buffer)?;
        }

        process_tick(&mut robots);
        tick += 1;
    }
}

fn process_tick(robots: &mut [Robot]) {
    // parse robot settings
    let bathroom_size = (101, 103);
    // simulate each robots position after 100 seconds
    robots
        .iter_mut()
        .for_each(|robot| robot.simulate_tick(bathroom_size));
}

fn display_robots(robots: &[Robot]) -> String {
    let display_size = (101, 103);
    let robot_positions = robots
        .iter()
        .map(|robot| (robot.position.0 as usize, robot.position.1 as usize))
        .collect();
    grid_display::GridDisplay::new('.', display_size, vec![('#', robot_positions)]).to_string()
}
