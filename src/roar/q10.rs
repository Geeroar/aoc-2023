#![allow(dead_code, unused_variables)]

use std::collections::{HashMap, HashSet, VecDeque};

use crate::utils::parser::{parse, FileLines};

/**
 * --- Day 10: Pipe Maze ---
 * I want to build a graph here. I'd like to use petgraph, but that feels like cheating.
 *
 * Going to try to build a graph from scratch.
 *
 */

type Location = (i64, i64);

#[derive(Debug, PartialEq)]
struct Edge {
    destination: Location,
}

#[derive(Debug)]
struct Node {
    location: Location,
    pipe: char,
    edges: Vec<Edge>,
    visited: bool,
}

#[derive(Debug)]
struct Graph {
    // All locations are unique so I'm hoping using them as a lookup key wil be good.
    nodes: HashMap<Location, Node>,
}

struct Input {
    start_node_location: Location,
    graph: Graph,
    grid_size: (i64, i64),
}

impl TryFrom<FileLines> for Input {
    type Error = std::io::Error;

    fn try_from(file_lines: FileLines) -> Result<Self, Self::Error> {
        let mut nodes: HashMap<Location, Node> = HashMap::new();
        let mut row: i64 = 0;
        let mut start_node_location = (0, 0);
        let mut width = 0;
        for line in file_lines {
            for (col, pipe) in line.chars().enumerate() {
                let col: i64 = col as i64;
                let location = (row, col);
                let node = Node {
                    location,
                    pipe,
                    edges: Vec::new(),
                    visited: false,
                };

                nodes.insert(location, node);
            }
            row += 1;
            width = line.len() as i64;
        }
        let height = row;
        // Connect the nodes
        let mut all_nodes = Vec::new();
        for key in nodes.keys() {
            all_nodes.push(*key);
        }
        // OMG this shit is hard to do in Rust when you don't really know what you're doing
        for node_key in all_nodes {
            let (row, col) = node_key;
            let south = (row + 1, col);
            let west = (row, col - 1);
            let east = (row, col + 1);
            let north = (row - 1, col);
            let has_north = nodes.contains_key(&north) && nodes.get(&north).unwrap().pipe != '.';
            let has_south = nodes.contains_key(&south) && nodes.get(&south).unwrap().pipe != '.';
            let has_east = nodes.contains_key(&east) && nodes.get(&east).unwrap().pipe != '.';
            let has_west = nodes.contains_key(&west) && nodes.get(&west).unwrap().pipe != '.';
            let node = nodes.get_mut(&node_key).unwrap();
            match node.pipe {
                '|' => {
                    if has_north {
                        node.edges.push(Edge { destination: north });
                    }
                    if has_south {
                        node.edges.push(Edge { destination: south });
                    }
                }
                '-' => {
                    if has_east {
                        node.edges.push(Edge { destination: east });
                    }
                    if has_west {
                        node.edges.push(Edge { destination: west });
                    }
                }
                'L' => {
                    if has_north {
                        node.edges.push(Edge { destination: north });
                    }
                    if has_east {
                        node.edges.push(Edge { destination: east });
                    }
                }
                'J' => {
                    if has_north {
                        node.edges.push(Edge { destination: north });
                    }
                    if has_west {
                        node.edges.push(Edge { destination: west });
                    }
                }
                '7' => {
                    if has_south {
                        node.edges.push(Edge { destination: south });
                    }
                    if has_west {
                        node.edges.push(Edge { destination: west });
                    }
                }
                'F' => {
                    if has_south {
                        node.edges.push(Edge { destination: south });
                    }
                    if has_east {
                        node.edges.push(Edge { destination: east });
                    }
                }
                'S' => {
                    start_node_location.0 = node.location.0;
                    start_node_location.1 = node.location.1;
                }
                '.' => {}
                _ => panic!("Unexpected character in input file: {}", node.pipe),
            }
        }

        // Check which nodes connect to start node and update start node edges
        let sl = start_node_location;
        let surrounding_start = [
            (sl.0 - 1, sl.1),
            (sl.0 + 1, sl.1),
            (sl.0, sl.1 - 1),
            (sl.0, sl.1 + 1),
        ];
        let mut start_node_edges = Vec::new();

        for node_location in surrounding_start {
            if nodes.contains_key(&node_location) {
                // Check the edges of the north node to see if they contain start node
                let north_node = nodes.get_mut(&node_location).unwrap();
                for edge in north_node.edges.iter() {
                    if edge.destination == sl {
                        start_node_edges.push(Edge {
                            destination: north_node.location,
                        });
                    }
                }
            }
        }

        nodes.insert(
            start_node_location,
            Node {
                location: start_node_location,
                pipe: 'S',
                edges: start_node_edges,
                visited: false,
            },
        );

        Ok(Input {
            graph: Graph { nodes },
            start_node_location,
            grid_size: (height, width),
        })
    }
}

fn find_longest_loop(graph: &mut Graph, start: Location) -> (i64, HashSet<Location>) {
    let mut pipe_loop = HashSet::new();
    let mut stack = VecDeque::new();
    let mut max_length = 0;

    stack.push_back((start, 0, HashSet::new()));

    while let Some((current, length, mut path_visited)) = stack.pop_back() {
        if current == start && !path_visited.is_empty() {
            // We have found a loop back to the start
            max_length = std::cmp::max(max_length, length);
            continue;
        }

        if path_visited.contains(&current) {
            // Skip if already visited in the current path
            continue;
        }

        path_visited.insert(current);
        pipe_loop.insert(current);

        if let Some(node) = graph.nodes.get(&current) {
            for edge in &node.edges {
                let new_path_visited = path_visited.clone();
                stack.push_back((edge.destination, length + 1, new_path_visited));
            }
        }
    }

    (max_length, pipe_loop)
}

fn part_1(input_file: &str) -> std::io::Result<i64> {
    let mut input = parse::<Input>(input_file)?;
    // Find the longest loop in the graph
    // Divide that by two to get the longest distance away from the start node.
    let longest_loop = find_longest_loop(&mut input.graph, input.start_node_location).0;
    Ok((longest_loop + 1) / 2)
}

fn get_points_inside_loop(graph: &Graph, loop_points: &HashSet<Location>) -> HashSet<Location> {
    /*
       Implementation of https://en.wikipedia.org/wiki/Point_in_polygon
    */
    let points_not_part_of_loop: Vec<&(i64, i64)> = graph
        .nodes
        .iter()
        // Filter to only get dot points outside the loop
        .filter(|(location, node)| !loop_points.contains(location))
        .map(|(location, _)| location)
        .collect();
    let mut point_inside_loop = HashSet::new();

    // https://en.wikipedia.org/wiki/Even%E2%80%93odd_rule
    for location in points_not_part_of_loop {
        if loop_points
            .iter()
            .filter(|location_of_point_in_loop| {
                location_of_point_in_loop.1 == location.1
                    && location_of_point_in_loop.0 < location.0
                    && ['-', '7', 'J'].contains(&graph.nodes[location_of_point_in_loop].pipe)
            })
            .count()
            .min(
                loop_points
                    .iter()
                    .filter(|p| {
                        p.1 == location.1
                            && p.0 < location.0
                            && ['L', 'F', '-'].contains(&graph.nodes[p].pipe)
                    })
                    .count(),
            )
            % 2
            == 1
        {
            point_inside_loop.insert(*location);
        }
    }
    point_inside_loop
}

fn part_2(input_file: &str) -> std::io::Result<i64> {
    let mut input = parse::<Input>(input_file)?;

    let main_loop = find_longest_loop(&mut input.graph, input.start_node_location).1;
    let points_inside_loop = get_points_inside_loop(&input.graph, &main_loop);

    Ok(points_inside_loop.len() as i64)
}

#[cfg(test)]
mod tests {
    use super::{part_1, part_2};

    const INPUT: &str = "input/roar/q10_input.txt";
    const INPUT_SAMPLE: &str = "input/roar/q10_sample.txt";

    #[test]
    fn roar_q10_p1_sample() {
        let result = part_1(INPUT_SAMPLE);
        assert_eq!(result.unwrap(), 8);
    }

    #[test]
    fn roar_q10_p1_main() {
        let result = part_1(INPUT);
        assert_eq!(result.unwrap(), 6815);
    }

    #[test]
    fn roar_q10_p2_sample() {
        let result = part_2(INPUT_SAMPLE);
        assert_eq!(result.unwrap(), 1);
    }

    #[ignore = "Takes too long"]
    #[test]
    fn roar_q10_p2_main() {
        let result = part_2(INPUT);
        // Answer currently unknown
        assert_eq!(result.unwrap(), 269);
    }
}
