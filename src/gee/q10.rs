#![allow(dead_code, unused_variables)]

use crate::utils::parser::{parse, FileLines};
use std::collections::HashSet;

type Step = (usize, usize, Direction);

#[derive(Debug)]
struct Input {
    grid: Vec<Vec<Tile>>,
    start: (usize, usize),
}

#[derive(Debug, Eq, PartialEq)]
enum Tile {
    AnimalStart,
    NorthToSouth,
    NorthToEast,
    NorthToWest,
    SouthToWest,
    SouthToEast,
    WestToEast,
    Ground,
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl TryFrom<FileLines> for Input {
    type Error = std::io::Error;

    fn try_from(lines: FileLines) -> Result<Self, Self::Error> {
        let mut grid = Vec::new();
        let mut start = (0, 0);
        for (i, line) in lines.enumerate() {
            let mut row = Vec::new();
            for (j, c) in line.chars().enumerate() {
                match c {
                    '|' => row.push(Tile::NorthToSouth),
                    'L' => row.push(Tile::NorthToEast),
                    'J' => row.push(Tile::NorthToWest),
                    '7' => row.push(Tile::SouthToWest),
                    'F' => row.push(Tile::SouthToEast),
                    'S' => {
                        row.push(Tile::AnimalStart);
                        start = (i, j);
                    }
                    '-' => row.push(Tile::WestToEast),
                    _ => row.push(Tile::Ground),
                }
            }
            grid.push(row);
        }
        Ok(Input { grid, start })
    }
}

impl Input {
    fn furthest_from_start(&self) -> u32 {
        let mut steps = 1;
        let (mut a, mut b) = self.paths_from_start();
        while (a.0, a.1) != (b.0, b.1) {
            (a, b) = (self.next_step(a), self.next_step(b));
            steps += 1;
        }
        steps
    }

    fn enclosed_area(&self) -> usize {
        let mut loop_points: HashSet<(usize, usize)> = HashSet::new();
        let mut inner_points: HashSet<(usize, usize)> = HashSet::new();
        let mut step = self.paths_from_start().0;
        loop_points.insert(self.start);
        loop_points.insert((step.0, step.1));
        while (step.0, step.1) != self.start {
            step = self.next_step(step);
            loop_points.insert((step.0, step.1));
        }
        step = self.paths_from_start().0;
        while (step.0, step.1) != self.start {
            match (step, &self.grid[step.0][step.1]) {
                ((r, c, Direction::North), Tile::NorthToSouth) => {
                    if c > 0 && !loop_points.contains(&(r, c - 1)) {
                        inner_points.insert((r, c - 1));
                    }
                }
                ((r, c, Direction::South), Tile::NorthToSouth) => {
                    if c < self.grid[0].len() - 1 && !loop_points.contains(&(r, c + 1)) {
                        inner_points.insert((r, c + 1));
                    }
                }
                ((r, c, Direction::West), Tile::WestToEast) => {
                    if r < self.grid.len() && !loop_points.contains(&(r + 1, c)) {
                        inner_points.insert((r + 1, c));
                    }
                }
                ((r, c, Direction::East), Tile::WestToEast) => {
                    if r > 0 && !loop_points.contains(&(r - 1, c)) {
                        inner_points.insert((r - 1, c));
                    }
                }
                ((r, c, Direction::East), Tile::SouthToEast) => {
                    if r > 0 && !loop_points.contains(&(r - 1, c)) {
                        inner_points.insert((r - 1, c));
                    }
                    if c > 0 && !loop_points.contains(&(r, c - 1)) {
                        inner_points.insert((r, c - 1));
                    }
                }
                ((r, c, Direction::West), Tile::NorthToWest) => {
                    if c < self.grid[0].len() - 1 && !loop_points.contains(&(r, c + 1)) {
                        inner_points.insert((r, c + 1));
                    }
                    if r < self.grid.len() && !loop_points.contains(&(r + 1, c)) {
                        inner_points.insert((r + 1, c));
                    }
                }
                ((r, c, Direction::North), Tile::NorthToEast) => {
                    if r < self.grid.len() && !loop_points.contains(&(r + 1, c)) {
                        inner_points.insert((r + 1, c));
                    }
                    if c > 0 && !loop_points.contains(&(r, c - 1)) {
                        inner_points.insert((r, c - 1));
                    }
                }
                ((r, c, Direction::South), Tile::SouthToWest) => {
                    if r > 0 && !loop_points.contains(&(r - 1, c)) {
                        inner_points.insert((r - 1, c));
                    }
                    if c < self.grid[0].len() - 1 && !loop_points.contains(&(r, c + 1)) {
                        inner_points.insert((r, c + 1));
                    }
                }
                _ => (),
            }
            step = self.next_step(step);
        }

        let mut inside_count = 0;
        while inside_count != inner_points.len() {
            inside_count = inner_points.len();
            for point in &inner_points.clone() {
                for connected_point in self.expand_point_within_boundary(point, &loop_points) {
                    inner_points.insert(connected_point);
                }
            }
        }
        self.print_grid(&loop_points, &inner_points);
        inner_points.len()
    }

    fn print_grid(
        &self,
        loop_points: &HashSet<(usize, usize)>,
        highlight_points: &HashSet<(usize, usize)>,
    ) {
        for i in 0..self.grid.len() {
            for j in 0..self.grid[0].len() {
                if loop_points.contains(&(i, j)) {
                    match self.grid[i][j] {
                        Tile::AnimalStart => print!("S"),
                        Tile::NorthToEast => print!("\u{2514}"),
                        Tile::NorthToSouth => print!("\u{2502}"),
                        Tile::NorthToWest => print!("\u{2518}"),
                        Tile::SouthToEast => print!("\u{250C}"),
                        Tile::SouthToWest => print!("\u{2510}"),
                        Tile::WestToEast => print!("\u{2500}"),
                        _ => print!("?"),
                    }
                } else if highlight_points.contains(&(i, j)) {
                    print!("O");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    fn expand_point_within_boundary(
        &self,
        source_point: &(usize, usize),
        boundary_points: &HashSet<(usize, usize)>,
    ) -> Vec<(usize, usize)> {
        let mut result = Vec::new();
        let &(r, c) = source_point;
        if r > 0 {
            if !boundary_points.contains(&(r - 1, c)) {
                result.push((r - 1, c));
            }
            if c > 0 && !boundary_points.contains(&(r - 1, c - 1)) {
                result.push((r - 1, c - 1));
            }
            if c < self.grid[0].len() - 1 && !boundary_points.contains(&(r - 1, c + 1)) {
                result.push((r - 1, c + 1));
            }
        }
        if r < self.grid.len() - 1 {
            if !boundary_points.contains(&(r + 1, c)) {
                result.push((r + 1, c));
            }
            if c > 0 && !boundary_points.contains(&(r + 1, c - 1)) {
                result.push((r + 1, c - 1));
            }
            if c < self.grid[0].len() - 1 && !boundary_points.contains(&(r + 1, c + 1)) {
                result.push((r + 1, c + 1));
            }
        }
        if c > 0 && !boundary_points.contains(&(r, c - 1)) {
            result.push((r, c - 1));
        }
        if c < self.grid[0].len() - 1 && !boundary_points.contains(&(r, c + 1)) {
            result.push((r, c + 1));
        }
        result
    }

    fn next_step(&self, path: Step) -> Step {
        let (row, col, from_direction) = path;
        match (from_direction, &self.grid[row][col]) {
            (Direction::North, Tile::NorthToWest) => (row, col - 1, Direction::East),
            (Direction::North, Tile::NorthToEast) => (row, col + 1, Direction::West),
            (Direction::North, Tile::NorthToSouth) => (row + 1, col, Direction::North),
            (Direction::South, Tile::NorthToSouth) => (row - 1, col, Direction::South),
            (Direction::South, Tile::SouthToWest) => (row, col - 1, Direction::East),
            (Direction::South, Tile::SouthToEast) => (row, col + 1, Direction::West),
            (Direction::West, Tile::NorthToWest) => (row - 1, col, Direction::South),
            (Direction::West, Tile::SouthToWest) => (row + 1, col, Direction::North),
            (Direction::West, Tile::WestToEast) => (row, col + 1, Direction::West),
            (Direction::East, Tile::NorthToEast) => (row - 1, col, Direction::South),
            (Direction::East, Tile::SouthToEast) => (row + 1, col, Direction::North),
            (Direction::East, Tile::WestToEast) => (row, col - 1, Direction::East),
            _ => panic!(
                "Bad next step: ({:?}, {:?})",
                from_direction, self.grid[row][col]
            ),
        }
    }

    fn paths_from_start(&self) -> (Step, Step) {
        let (row, col) = self.start;
        let mut paths = Vec::new();
        if let Some(p) = self.path_north(self.start) {
            paths.push(p);
        }
        if let Some(p) = self.path_south(self.start) {
            paths.push(p);
        }
        if let Some(p) = self.path_west(self.start) {
            paths.push(p);
        }
        if let Some(p) = self.path_east(self.start) {
            paths.push(p);
        }
        (paths[0], paths[1])
    }

    fn path_north(&self, point: (usize, usize)) -> Option<Step> {
        let (r, c) = point;
        let grid = &self.grid;
        let tiles_above = vec![Tile::NorthToSouth, Tile::SouthToWest, Tile::SouthToEast];
        if r > 0 && tiles_above.contains(&grid[r - 1][c]) {
            Some((r - 1, c, Direction::South))
        } else {
            None
        }
    }

    fn path_south(&self, point: (usize, usize)) -> Option<Step> {
        let (r, c) = point;
        let grid = &self.grid;
        let tiles_below = vec![Tile::NorthToSouth, Tile::NorthToWest, Tile::NorthToEast];
        if r < grid.len() - 1 && tiles_below.contains(&grid[r + 1][c]) {
            Some((r + 1, c, Direction::North))
        } else {
            None
        }
    }

    fn path_west(&self, point: (usize, usize)) -> Option<Step> {
        let (r, c) = point;
        let grid = &self.grid;
        let tiles_left = vec![Tile::WestToEast, Tile::NorthToEast, Tile::SouthToEast];
        if c > 0 && tiles_left.contains(&grid[r][c - 1]) {
            Some((r, c - 1, Direction::East))
        } else {
            None
        }
    }

    fn path_east(&self, point: (usize, usize)) -> Option<Step> {
        let (r, c) = point;
        let grid = &self.grid;
        let tiles_right = vec![Tile::WestToEast, Tile::NorthToWest, Tile::SouthToWest];
        if c < grid[0].len() - 1 && tiles_right.contains(&grid[r][c + 1]) {
            Some((r, c + 1, Direction::West))
        } else {
            None
        }
    }
}

fn part_1(input_file: &str) -> std::io::Result<u32> {
    let input: Input = parse(input_file)?;
    Ok(input.furthest_from_start())
}

fn part_2(input_file: &str) -> std::io::Result<usize> {
    let input: Input = parse(input_file)?;
    Ok(input.enclosed_area())
}

#[cfg(test)]
mod tests {
    use super::{part_1, part_2};

    const INPUT: &str = "input/gee/q10_input.txt";
    const INPUT_SAMPLE: &str = "input/gee/q10_sample.txt";
    const INPUT_SAMPLE_2: &str = "input/gee/q10_sample_2.txt";

    #[test]
    fn gee_q10_p1_sample() {
        let result = part_1(INPUT_SAMPLE);
        assert_eq!(result.unwrap(), 8);
    }

    #[test]
    fn gee_q10_p1_main() {
        let result = part_1(INPUT);
        assert_eq!(result.unwrap(), 6968);
    }

    #[test]
    fn gee_q10_p2_sample() {
        let result = part_2(INPUT_SAMPLE_2);
        assert_eq!(result.unwrap(), 10);
    }

    #[test]
    fn gee_q10_p2_main() {
        let result = part_2(INPUT);
        assert_eq!(result.unwrap(), 413);
    }
}
