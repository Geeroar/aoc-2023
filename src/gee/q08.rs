use crate::utils::parser::FileLines;
use std::collections::HashMap;

#[derive(Debug)]
struct Input {
    _instructions: Vec<Direction>,
    _nodes: HashMap<String, (String, String)>,
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

impl TryFrom<FileLines> for Input {
    type Error = std::io::Error;

    fn try_from(mut _lines: FileLines) -> Result<Self, Self::Error> {
        let _instructions = _lines
            .next_result()?
            .chars()
            .map(|c| match c {
                'L' => Direction::Left,
                'R' => Direction::Right,
                _ => panic!("Bad direction: {}", c),
            })
            .collect();
        _lines.next();
        let mut _nodes = HashMap::new();
        for line in _lines {
            if let Some((src, dest)) = line.split_once(" = ") {
                if let Some((l, r)) = dest.split_once(", ") {
                    _nodes.insert(
                        String::from(src),
                        (String::from(&l[1..]), String::from(&r[..3])),
                    );
                }
            }
        }
        Ok(Input {
            _instructions,
            _nodes,
        })
    }
}

fn _part_1(input_file: &str) -> std::io::Result<u32> {
    let input = Input::try_from(FileLines::new(input_file)?)?;
    let mut steps = 0;
    let mut i = 0;
    let (instructions, nodes) = (input._instructions, input._nodes);
    let mut current_node = &String::from("AAA");
    loop {
        current_node = match instructions[i] {
            Direction::Left => nodes.get(current_node).map(|(l, _)| l).unwrap(),
            Direction::Right => nodes.get(current_node).map(|(_, r)| r).unwrap(),
        };
        steps += 1;
        if current_node == "ZZZ" {
            break;
        } else {
            i = match i {
                n if n >= instructions.len() - 1 => 0,
                n => n + 1,
            };
        }
    }
    Ok(steps)
}

pub fn _part_2(input_file: &str) -> std::io::Result<u64> {
    let input = Input::try_from(FileLines::new(input_file)?)?;
    let mut steps = 0;
    let mut i = 0;
    let (instructions, nodes) = (input._instructions, input._nodes);
    let mut current_nodes: Vec<(&str, &str, Vec<(&str, u64)>)> = nodes
        .keys()
        .filter(|s| s.ends_with('A'))
        .map(|s| (s.as_str(), s.as_str(), vec![("", 0)]))
        .collect();
    current_nodes.sort();
    let mut cycles: HashMap<&str, u64> = HashMap::new();
    loop {
        let instruction = &instructions[i];
        current_nodes = current_nodes
            .into_iter()
            .map(|(s, n, mut v)| match instruction {
                Direction::Left => {
                    let x = nodes.get(n).map(|(l, _)| l).unwrap().as_str();
                    (s, x, v)
                }
                Direction::Right => {
                    let x = nodes.get(n).map(|(_, r)| r).unwrap().as_str();
                    if x.ends_with('Z') {
                        v.push((x, steps + 1));
                        if !cycles.contains_key(s) {
                            cycles.insert(s, steps + 1);
                        }
                    }
                    (s, x, v)
                }
            })
            .collect();
        steps += 1;
        if cycles.len() == current_nodes.len() {
            break;
        } else {
            i = match i {
                n if n >= instructions.len() - 1 => 0,
                n => n + 1,
            };
        }
    }
    steps = 0;
    let cycle_sizes: Vec<u64> = cycles.values().map(|&x| x).collect();
    let largest_jump = cycle_sizes.iter().max().unwrap();
    loop {
        steps += largest_jump;
        if cycle_sizes.iter().all(|&x| steps % x == 0) {
            break;
        }
    }
    Ok(steps)
}

#[cfg(test)]
mod tests {
    use super::{_part_1, _part_2};

    const INPUT: &str = "input/gee/q08_input.txt";
    const INPUT_SAMPLE_1: &str = "input/gee/q08_sample.txt";
    const INPUT_SAMPLE_2: &str = "input/gee/q08_sample_2.txt";
    const INPUT_SAMPLE_3: &str = "input/gee/q08_sample_3.txt";

    #[test]
    fn gee_q08_p1_sample_1() {
        let result = _part_1(INPUT_SAMPLE_1);
        assert_eq!(result.unwrap(), 2);
    }

    #[test]
    fn gee_q08_p1_sample_2() {
        let result = _part_1(INPUT_SAMPLE_2);
        assert_eq!(result.unwrap(), 6);
    }

    #[test]
    fn gee_q08_p1_main() {
        let result = _part_1(INPUT);
        assert_eq!(result.unwrap(), 18113);
    }

    #[test]
    fn gee_q08_p2_sample() {
        let result = _part_2(INPUT_SAMPLE_3);
        assert_eq!(result.unwrap(), 6);
    }

    #[ignore]
    #[test]
    fn gee_q08_p2_main() {
        let result = _part_2(INPUT);
        assert_eq!(result.unwrap(), 12315788159977);
    }
}
