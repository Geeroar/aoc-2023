#![allow(dead_code, unused_variables)]

use crate::utils::parser::{parse, FileLines};
use std::collections::{BinaryHeap, HashMap, HashSet};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Input {
    grid: Vec<Vec<usize>>,
}

type Node = (usize, usize);
type Cost = usize;
type Value = usize;
type Streak = usize;
type Path = (Node, Cost, Direction, Streak);
type OpenSetItem = (Node, Direction, Streak);
type QueueItem = (Value, Path);

impl TryFrom<FileLines> for Input {
    type Error = std::io::Error;

    fn try_from(lines: FileLines) -> Result<Self, Self::Error> {
        let grid = lines
            .map(|l| {
                l.chars()
                    .map(|c| c.to_digit(10).unwrap() as usize)
                    .collect()
            })
            .collect();
        Ok(Input { grid })
    }
}

fn path_value(grid: &[Vec<usize>], path: Path, target: Node) -> usize {
    let ((r, c), g, _, _) = path;
    let h = target.0 - r + target.1 - c;
    usize::MAX - (g + h)
}

fn find_path(grid: &[Vec<usize>], start: Node, target: Node, is_ultra: bool) -> usize {
    let mut queue = BinaryHeap::<QueueItem>::new();
    let mut open_set = HashSet::<OpenSetItem>::new();
    let mut g_scores = HashMap::new();
    let start_right = (start, 0, Direction::Right, 0);
    let start_down = (start, 0, Direction::Down, 0);
    queue.push((path_value(grid, start_right, target), start_right));
    queue.push((path_value(grid, start_down, target), start_down));
    while let Some((value, path)) = queue.pop() {
        let (node, cost, direction, streak) = path;
        if node == target && (!is_ultra || streak >= 4) {
            return cost;
        }
        open_set.remove(&(node, direction, streak));
        for next_step in valid_next_steps(grid, path, target, is_ultra) {
            let (next_node, next_cost, next_direction, next_streak) = next_step;
            let open_set_item = (next_node, next_direction, next_streak);
            let current_best = *g_scores.get(&open_set_item).unwrap_or(&usize::MAX);
            if next_cost < current_best {
                g_scores.insert(open_set_item, next_cost);
                if !open_set.contains(&open_set_item) {
                    let value = path_value(grid, next_step, target);
                    open_set.insert(open_set_item);
                    queue.push((value, next_step));
                }
            }
        }
    }
    usize::MAX
}

fn try_to_take_step(
    grid: &[Vec<usize>],
    path: Path,
    direction: Direction,
    is_ultra: bool,
) -> Option<Path> {
    let ((r, c), last_cost, last_direction, last_streak) = path;
    if !is_ultra && direction == last_direction && last_streak >= 3 {
        return None;
    }
    if is_ultra && direction == last_direction && last_streak >= 10 {
        return None;
    }
    if is_ultra && direction != last_direction && last_streak < 4 {
        return None;
    }
    let next_node = match direction {
        Direction::Up => {
            if r == 0 || last_direction == Direction::Down {
                return None;
            } else {
                (r - 1, c)
            }
        }
        Direction::Down => {
            if r >= grid.len() - 1 || last_direction == Direction::Up {
                return None;
            } else {
                (r + 1, c)
            }
        }
        Direction::Left => {
            if c == 0 || last_direction == Direction::Right {
                return None;
            } else {
                (r, c - 1)
            }
        }
        Direction::Right => {
            if c >= grid[0].len() - 1 || last_direction == Direction::Left {
                return None;
            } else {
                (r, c + 1)
            }
        }
    };
    let streak = if direction == last_direction {
        last_streak + 1
    } else {
        1
    };
    Some((
        next_node,
        last_cost + grid[next_node.0][next_node.1],
        direction,
        streak,
    ))
}

fn valid_next_steps(grid: &[Vec<usize>], path: Path, target: Node, is_ultra: bool) -> Vec<Path> {
    if path.0 == target {
        vec![]
    } else {
        let mut candidates = Vec::new();
        for direction in [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ] {
            if let Some(p) = try_to_take_step(grid, path, direction, is_ultra) {
                candidates.push(p);
            }
        }
        candidates
    }
}

fn part_1(input_file: &str) -> std::io::Result<usize> {
    let input: Input = parse(input_file)?;
    let grid = &input.grid;
    Ok(find_path(
        grid,
        (0, 0),
        (grid.len() - 1, grid[0].len() - 1),
        false,
    ))
}

fn part_2(input_file: &str) -> std::io::Result<usize> {
    let input: Input = parse(input_file)?;
    let grid = &input.grid;
    Ok(find_path(
        grid,
        (0, 0),
        (grid.len() - 1, grid[0].len() - 1),
        true,
    ))
}

#[cfg(test)]
mod tests {
    use super::{part_1, part_2};

    const INPUT: &str = "input/gee/q17_input.txt";
    const INPUT_SAMPLE: &str = "input/gee/q17_sample.txt";

    #[test]
    fn gee_q17_p1_sample() {
        let result = part_1(INPUT_SAMPLE);
        assert_eq!(result.unwrap(), 102);
    }

    #[test]
    fn gee_q17_p1_main() {
        let result = part_1(INPUT);
        assert_eq!(result.unwrap(), 1001);
    }

    #[test]
    fn gee_q17_p2_sample() {
        let result = part_2(INPUT_SAMPLE);
        assert_eq!(result.unwrap(), 94);
    }

    #[ignore = "takes too long"]
    #[test]
    fn gee_q17_p2_main() {
        let result = part_2(INPUT);
        assert_eq!(result.unwrap(), 1197);
    }
}
