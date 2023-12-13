use std::collections::HashMap;

use crate::utils::parser::FileLines;
use num::integer::lcm;

struct Input {
    _directions: Vec<char>,
    _network: HashMap<String, (String, String)>,
    _ends_with_a: Vec<String>,
    _ends_with_z: Vec<String>,
}

impl TryFrom<FileLines> for Input {
    type Error = std::io::Error;

    fn try_from(_lines: FileLines) -> Result<Self, Self::Error> {
        let mut directions = Vec::new();
        let mut network = HashMap::new();
        let mut ends_with_a = Vec::new();
        let mut ends_with_z = Vec::new();
        for (i, line) in _lines.enumerate() {
            if i == 0 {
                directions = line.chars().collect();
            } else if i > 1 {
                let mut _line = line.split_once(" = ").unwrap();
                let node_name = _line.0.to_string();
                if node_name.ends_with('A') {
                    ends_with_a.push(node_name.clone());
                }
                if node_name.ends_with('Z') {
                    ends_with_z.push(node_name.clone());
                }
                let value_brackets_removed = &_line.1[1.._line.1.len() - 1];
                let value_tuple = value_brackets_removed.split_once(", ").unwrap();
                let left_node = value_tuple.0.to_string();
                let right_node = value_tuple.1.to_string();
                network.insert(node_name, (left_node, right_node));
            }
        }
        Ok(Input {
            _directions: directions,
            _network: network,
            _ends_with_a: ends_with_a,
            _ends_with_z: ends_with_z,
        })
    }
}

fn _get_steps_betwixt_nodes(
    directions: &Vec<char>,
    network: &HashMap<String, (String, String)>,
    start: &str,
    end: &str,
    ends_with_z_check: bool,
) -> u64 {
    let mut current = start;
    let mut steps = 0;
    let mut direction_index = 0;
    let mut finish = false;
    while !finish {
        let direction = directions[direction_index];
        direction_index = if direction_index == directions.len() - 1 {
            0
        } else {
            direction_index + 1
        };
        steps += 1;
        if direction == 'L' {
            current = &network[current].0;
        } else if direction == 'R' {
            current = &network[current].1;
        }
        finish = if ends_with_z_check {
            current.ends_with('Z')
        } else {
            current == end
        }
    }
    steps
}

fn _part_1(input_file: &str) -> std::io::Result<u64> {
    let input = Input::try_from(FileLines::new(input_file)?)?;
    let start = "AAA";
    let end = "ZZZ";
    Ok(_get_steps_betwixt_nodes(
        &input._directions,
        &input._network,
        start,
        end,
        false,
    ))
}

// This was my first, terrible idea
fn _part_2_bad(input_file: &str) -> std::io::Result<u64> {
    let input = Input::try_from(FileLines::new(input_file)?)?;
    let mut steps = 0;
    let mut direction_index = 0;
    let mut all_nodes_end_with_z = false;
    let mut current_nodes = input._ends_with_a.clone();
    while !all_nodes_end_with_z {
        let direction = input._directions[direction_index];
        direction_index = if direction_index == input._directions.len() - 1 {
            0
        } else {
            direction_index + 1
        };
        steps += 1;

        all_nodes_end_with_z = true;
        for node_name in &mut current_nodes {
            let node = &input._network[node_name];
            if direction == 'L' {
                *node_name = node.0.clone();
            } else if direction == 'R' {
                *node_name = node.1.clone();
            }

            if !node_name.ends_with('Z') {
                all_nodes_end_with_z = false;
            }
        }
    }

    Ok(steps)
}

fn _part_2(input_file: &str) -> std::io::Result<u64> {
    let input = Input::try_from(FileLines::new(input_file)?)?;
    let directions = &input._directions;

    println!("{:?}", input._ends_with_a);
    println!("{:?}", input._ends_with_z);
    let steps_counts: Vec<Vec<u64>> = input
        ._ends_with_a
        .iter()
        .map(|node| {
            let current_node = node.clone();
            let mut current_steps = Vec::new();
            for end_node in input._ends_with_z.iter() {
                let steps = _get_steps_betwixt_nodes(
                    directions,
                    &input._network,
                    &current_node,
                    end_node,
                    true,
                );
                current_steps.push(steps);
            }

            current_steps
        })
        .collect();

    println!("{:?}", steps_counts);

    let result_lcm = steps_counts.into_iter().flatten().reduce(lcm).unwrap_or(0);

    Ok(result_lcm)
}

#[cfg(test)]
mod tests {
    use super::{_part_1, _part_2};

    const INPUT: &str = "input/roar/q08_input.txt";
    const INPUT_SAMPLE: &str = "input/roar/q08_sample.txt";
    const INPUT_SAMPLE_WITH_CYCLE: &str = "input/roar/q08_sample_with_cycle.txt";
    const INPUT_SAMPLE_P2: &str = "input/roar/q08_sample_p2.txt";

    #[test]
    fn roar_q08_p1_sample() {
        let result = _part_1(INPUT_SAMPLE);
        assert_eq!(result.unwrap(), 2);
    }

    #[test]
    fn roar_q08_p1_sample_with_cycle() {
        let result = _part_1(INPUT_SAMPLE_WITH_CYCLE);
        assert_eq!(result.unwrap(), 6);
    }

    #[test]
    fn roar_q08_p1_main() {
        let result = _part_1(INPUT);
        assert_eq!(result.unwrap(), 13019);
    }

    #[test]
    fn roar_q08_p2_sample() {
        let result = _part_2(INPUT_SAMPLE_P2);
        assert_eq!(result.unwrap(), 6);
    }

    #[test]
    fn roar_q08_p2_main() {
        let result = _part_2(INPUT);
        assert_eq!(result.unwrap(), 13524038372771);
    }
}
