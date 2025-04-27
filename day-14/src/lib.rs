pub struct Robot {
    pub position: (i32, i32),
    pub velocity: (i32, i32),
}

pub fn parse_robots(input: &str) -> Vec<Robot> {
    input
        .lines()
        .map(|line| {
            let (start_pos, velocity) = line.strip_prefix("p=").unwrap().split_once(" v=").unwrap();
            let start_pos = start_pos
                .split_once(',')
                .map(|(x, y)| (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap()))
                .unwrap();
            let velocity = velocity
                .split_once(',')
                .map(|(x, y)| (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap()))
                .unwrap();
            Robot {
                position: start_pos,
                velocity,
            }
        })
        .collect()
}

impl Robot {
    /// Simulate all ticks at once with simulate_ticks instead!
    pub fn simulate_tick(&mut self, bathroom_size: (i32, i32)) {
        self.position = (
            (self.position.0 + self.velocity.0).rem_euclid(bathroom_size.0),
            (self.position.1 + self.velocity.1).rem_euclid(bathroom_size.1),
        );
    }
    /*
    pub fn simulate_ticks(&mut self, bathroom_size: (i32, i32), ticks: usize) {
        for _ in 0..ticks {
            self.simulate_tick(bathroom_size);
        }
    }
    */
    pub fn simulate_ticks(&mut self, bathroom_size: (i32, i32), ticks: usize) {
        self.position = (
            (self.position.0 + (self.velocity.0 * ticks as i32)).rem_euclid(bathroom_size.0),
            (self.position.1 + (self.velocity.1 * ticks as i32)).rem_euclid(bathroom_size.1),
        );
    }

    /// maps the robot to a quadrant (0, 1, 2, 3) and returns None if the robot is on one of the
    /// center lines of the bathroom
    pub fn map_robot_to_quadrant(&self, bathroom_size: (i32, i32)) -> Option<usize> {
        if self.position.0 == (bathroom_size.0 / 2) || self.position.1 == (bathroom_size.1 / 2) {
            return None;
        }
        let horizontal_quadrant = self.position.0 / ((bathroom_size.0 + 1) / 2); // 0 or 1
        let vertical_quadrant = self.position.1 / ((bathroom_size.1 + 1) / 2) * 2; // 0 or 2
        Some((horizontal_quadrant + vertical_quadrant) as usize)
    }
}

#[cfg(test)]
mod tests {
    use crate::Robot;

    #[test]
    fn test_quadrant() {
        let bathroom_size = (11, 7);
        assert_eq!(
            Some(0),
            Robot {
                position: (0, 1),
                velocity: (-1, 1)
            }
            .map_robot_to_quadrant(bathroom_size)
        );
        assert_eq!(
            None,
            Robot {
                position: (5, 5),
                velocity: (-1, 1)
            }
            .map_robot_to_quadrant(bathroom_size)
        );
        assert_eq!(
            Some(3),
            Robot {
                position: (8, 5),
                velocity: (-1, 1)
            }
            .map_robot_to_quadrant(bathroom_size)
        );
        assert_eq!(
            Some(3),
            Robot {
                position: (10, 6),
                velocity: (-1, 1)
            }
            .map_robot_to_quadrant(bathroom_size)
        );
    }
}
