use aoc_util::direction::Direction;
use aoc_util::grid::Grid;
use rustc_hash::FxHashSet;
use std::collections::VecDeque;

#[derive(Debug)]
pub struct Region {
    pub plant: char,
    pub plots: Vec<(usize, usize)>,
}
pub fn find_plot_perimeter(region: Region, grid: &Grid<char>) -> usize {
    let mut perimeter = 0;

    for plot in region.plots {
        let neighbors = grid.get_neighbors(plot);
        // get_neighbors filters anything outside of the grid -> automatically part of the perimeter
        if neighbors.len() < 4 {
            perimeter += 4 - neighbors.len();
        }
        perimeter += neighbors
            .iter()
            .filter(|position| grid.get(**position).unwrap() != &region.plant)
            .count()
    }

    perimeter
}

pub fn find_number_of_fences(region: Region, grid: &Grid<char>) -> usize {
    // find a starting point at the border of this region
    let mut border_settings = find_border_settings(&region, grid);
    let mut corners = 0;
    while let Some((start_pos, start_dir)) = border_settings.iter().next().cloned() {
        let start_dir = start_dir.cw();
        let (mut position, mut direction) = (start_pos, start_dir);

        loop {
            // check left -> match -> turn left; move; add a corner
            // println!("{:?} -> {:?} -> {corners}", position, direction);
            let turn_left = grid.get_direct_neighbor(position, direction.ccw());
            if turn_left.is_some() && turn_left.unwrap().1 == &region.plant {
                border_settings.remove(&(position, direction.ccw()));
                position = turn_left.unwrap().0;
                direction = direction.ccw();
                corners += 1;
            } else {
                let walk_ahead = grid.get_direct_neighbor(position, direction);
                // try walking straight ahead
                if walk_ahead.is_some() && walk_ahead.unwrap().1 == &region.plant {
                    border_settings.remove(&(position, direction.ccw()));
                    position = walk_ahead.unwrap().0;
                } else {
                    // if you can't -> you gotta turn right
                    border_settings.remove(&(position, direction));
                    border_settings.remove(&(position, direction.ccw()));
                    direction = direction.cw();
                    corners += 1;
                }
            }

            if position.eq(&start_pos) && direction.eq(&start_dir) {
                break;
            }
        }
    }

    corners
}

fn find_border_settings(
    region: &Region,
    grid: &Grid<char>,
) -> FxHashSet<((usize, usize), Direction)> {
    region
        .plots
        .iter()
        .flat_map(|plot| {
            Direction::CLOCKWISE.into_iter().filter_map(|dir| {
                let direct_neighbor = grid.get_direct_neighbor(*plot, dir);
                if direct_neighbor.is_none() || direct_neighbor.unwrap().1 != &region.plant {
                    return Some((*plot, dir));
                }
                None
            })
        })
        .collect()
}

pub fn find_regions(grid: &Grid<char>) -> Vec<Region> {
    let mut plots_in_regions: FxHashSet<(usize, usize)> = FxHashSet::default();
    let mut result = vec![];
    for position in grid.into_iter() {
        if !plots_in_regions.contains(&position) {
            let c = grid.get(position).unwrap();

            let region_plots = find_region(position, c, grid, &mut plots_in_regions);
            let region = Region {
                plant: *c,
                plots: region_plots,
            };
            result.push(region);
        }
    }

    result
}

pub fn find_region(
    position: (usize, usize),
    c: &char,
    grid: &Grid<char>,
    visited: &mut FxHashSet<(usize, usize)>,
) -> Vec<(usize, usize)> {
    let mut result = FxHashSet::default();
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    queue.push_front(position);
    while let Some(pos) = queue.pop_front() {
        if !visited.contains(&pos) && grid.get(pos).unwrap() == c {
            result.insert(pos);
            visited.insert(pos);
            grid.get_neighbors(pos)
                .iter()
                .filter(|neighbor| !visited.contains(neighbor))
                .for_each(|neighbor| queue.push_back(*neighbor));
        }
    }
    result.into_iter().collect()
}
