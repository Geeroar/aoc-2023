#![allow(dead_code, unused_variables)]

use std::collections::HashMap;
use std::collections::HashSet;

use crate::utils::parser::{parse, FileLines};

struct Input {
    wiring_diagram: HashMap<String, Vec<String>>,
}

impl TryFrom<FileLines> for Input {
    type Error = std::io::Error;

    fn try_from(lines: FileLines) -> Result<Self, Self::Error> {
        let mut wiring_diagram: HashMap<String, Vec<String>> = HashMap::new();

        for line in lines {
            let parts: Vec<&str> = line.split(":").map(|s| s.trim()).collect();
            let key = parts[0].to_owned();
            let values = parts[1]
                .split_whitespace()
                .map(|s| s.to_owned())
                .collect::<Vec<_>>();

            for value in &values {
                wiring_diagram
                    .entry(key.clone())
                    .or_default()
                    .push(value.clone());
                wiring_diagram
                    .entry(value.clone())
                    .or_default()
                    .push(key.clone());
            }
        }
        Ok(Input { wiring_diagram })
    }
}

fn dfs(
    node: &String,
    visited: &mut HashSet<String>,
    wiring_diagram: &HashMap<String, Vec<String>>,
) -> usize {
    visited.insert(node.clone());
    let mut size = 1; // Count the current node

    if let Some(neighbors) = wiring_diagram.get(node) {
        for neighbor in neighbors {
            if !visited.contains(neighbor) {
                size += dfs(neighbor, visited, wiring_diagram); // Recursively visit neighbors
            }
        }
    }

    size
}

fn disconnect_edge(node1: &str, node2: &str, diagram: &mut HashMap<String, Vec<String>>) {
    if let Some(neighbors) = diagram.get_mut(node1) {
        neighbors.retain(|x| x != node2);
    }
    if let Some(neighbors) = diagram.get_mut(node2) {
        neighbors.retain(|x| x != node1);
    }
}

fn divide_graph_into_groups(wiring_diagram: &HashMap<String, Vec<String>>) -> usize {
    let mut max_product = 0;

    let mut edges = Vec::new();
    for (node, neighbors) in wiring_diagram {
        for neighbor in neighbors {
            edges.push((node.clone(), neighbor.clone()));
        }
    }

    // Try disconnecting each combination of three edges
    let edge_count = edges.len();
    for i in 0..edge_count {
        for j in i + 1..edge_count {
            for k in j + 1..edge_count {
                // Create a copy of the wiring diagram
                let mut modified_diagram = wiring_diagram.clone();

                // Disconnect the selected edges
                disconnect_edge(&edges[i].0, &edges[i].1, &mut modified_diagram);
                disconnect_edge(&edges[j].0, &edges[j].1, &mut modified_diagram);
                disconnect_edge(&edges[k].0, &edges[k].1, &mut modified_diagram);

                // Disconnect the selected edges
                modified_diagram
                    .get_mut(&edges[i].0)
                    .unwrap()
                    .retain(|x| x != &edges[i].1);
                modified_diagram
                    .get_mut(&edges[i].1)
                    .unwrap()
                    .retain(|x| x != &edges[i].0);
                modified_diagram
                    .get_mut(&edges[j].0)
                    .unwrap()
                    .retain(|x| x != &edges[j].1);
                modified_diagram
                    .get_mut(&edges[j].1)
                    .unwrap()
                    .retain(|x| x != &edges[j].0);
                modified_diagram
                    .get_mut(&edges[k].0)
                    .unwrap()
                    .retain(|x| x != &edges[k].1);
                modified_diagram
                    .get_mut(&edges[k].1)
                    .unwrap()
                    .retain(|x| x != &edges[k].0);

                // Calculate the size of the groups
                let mut visited = HashSet::new();
                let mut group_sizes = Vec::new();
                for node in modified_diagram.keys() {
                    if !visited.contains(node) {
                        let size = dfs(node, &mut visited, &modified_diagram);
                        group_sizes.push(size);
                    }
                }

                if group_sizes.len() == 2 {
                    let product = group_sizes[0] * group_sizes[1];
                    if product > max_product {
                        max_product = product;
                    }
                }
            }
        }
    }

    max_product
}

fn part_1(input_file: &str) -> std::io::Result<usize> {
    let input: Input = parse(input_file)?;
    Ok(divide_graph_into_groups(&input.wiring_diagram))
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

    #[test]
    fn roar_q25_p1_sample() {
        let result = part_1(INPUT_SAMPLE);
        assert_eq!(result.unwrap(), 54);
    }

    #[test]
    fn roar_q25_p1_main() {
        let result = part_1(INPUT);
        // TODO: Find the actual value
        assert_eq!(result.unwrap(), 0);
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
