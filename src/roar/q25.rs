#![allow(dead_code, unused_variables)]
use crate::utils::parser::{parse, FileLines};
use std::collections::VecDeque;
use std::collections::{HashMap, HashSet};

struct Input {
    graph: HashMap<String, HashSet<String>>,
}

impl TryFrom<FileLines> for Input {
    type Error = std::io::Error;

    fn try_from(lines: FileLines) -> Result<Self, Self::Error> {
        let mut graph: HashMap<String, HashSet<String>> = HashMap::new();

        for line in lines {
            let parts: Vec<_> = line.split(": ").map(String::from).collect();
            let node = parts[0].clone();
            let neighbors = parts[1]
                .split_whitespace()
                .map(String::from)
                .collect::<HashSet<_>>();

            for neighbor in &neighbors {
                graph
                    .entry(node.clone())
                    .or_default()
                    .insert(neighbor.clone());
                graph
                    .entry(neighbor.clone())
                    .or_default()
                    .insert(node.clone());
            }
        }

        Ok(Input { graph })
    }
}

fn find_bridge(graph: &HashMap<String, HashSet<String>>) -> (String, String) {
    let mut paths: HashMap<(String, String), usize> = HashMap::new();

    for start in graph.keys() {
        let mut to_see = VecDeque::new();
        to_see.push_back(start.clone());
        let mut seen = HashSet::new();
        seen.insert(start.clone());

        while let Some(node) = to_see.pop_front() {
            for n in &graph[&node] {
                if !seen.contains(n) {
                    to_see.push_back(n.clone());
                    seen.insert(n.clone());
                    let edge = if n < &node {
                        (n.clone(), node.clone())
                    } else {
                        (node.clone(), n.clone())
                    };
                    *paths.entry(edge).or_default() += 1;
                }
            }
        }
    }

    paths.into_iter().max_by_key(|&(_, v)| v).unwrap().0
}

fn bfs_reach(graph: &HashMap<String, HashSet<String>>, start: &str) -> usize {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(start.to_string());

    while let Some(node) = queue.pop_front() {
        if !visited.insert(node.clone()) {
            continue;
        }

        if let Some(neighbors) = graph.get(&node) {
            for neighbor in neighbors {
                if !visited.contains(neighbor) {
                    queue.push_back(neighbor.clone());
                }
            }
        }
    }

    visited.len()
}

fn part_1(input_file: &str) -> std::io::Result<usize> {
    let mut input: Input = parse(input_file)?;

    for _ in 0..3 {
        let bridge = find_bridge(&input.graph);
        input.graph.get_mut(&bridge.0).unwrap().remove(&bridge.1);
        input.graph.get_mut(&bridge.1).unwrap().remove(&bridge.0);
    }

    let start = input.graph.keys().next().unwrap().clone();
    let group_size = bfs_reach(&input.graph, &start);
    Ok(group_size * (input.graph.len() - group_size))
}

fn part_2(input_file: &str) -> std::io::Result<usize> {
    let input: Input = parse(input_file)?;
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::{part_1, part_2};

    const INPUT: &str = "input/roar/q25_input.txt";
    const INPUT_SAMPLE: &str = "input/roar/q25_sample.txt";

    #[ignore]
    #[test]
    fn roar_q25_p1_sample() {
        let result = part_1(INPUT_SAMPLE);
        assert_eq!(result.unwrap(), 54);
    }

    #[ignore]
    #[test]
    fn roar_q25_p1_main() {
        let result = part_1(INPUT);
        // TODO: Find the actual value
        assert_eq!(result.unwrap(), 583632);
    }

    #[test]
    fn roar_q25_p2_sample() {
        let result = part_2(INPUT_SAMPLE);
        assert_eq!(result.unwrap(), 0);
    }

    #[test]
    fn roar_q25_p2_main() {
        let result = part_2(INPUT);
        assert_eq!(result.unwrap(), 0);
    }
}
