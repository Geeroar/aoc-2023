#![allow(dead_code, unused_variables)]

use std::collections::HashMap;

use crate::utils::parser::{parse, FileLines};

struct Input {
    workflows: HashMap<String, Vec<String>>,
    part_ratings: Vec<HashMap<String, u32>>,
}

impl TryFrom<FileLines> for Input {
    type Error = std::io::Error;

    fn try_from(lines: FileLines) -> Result<Self, Self::Error> {
        let mut workflows_parsed = false;
        let mut workflows = HashMap::new();
        let mut part_ratings = Vec::new();

        for line in lines {
            if line.is_empty() {
                workflows_parsed = true;
                continue;
            }
            if workflows_parsed {
                // parse part ratings
                let mut part_rating: HashMap<String, u32> = HashMap::new();
                let input = line.trim_matches('{').trim_matches('}');
                let pairs: Vec<&str> = input.split(',').collect();
                for pair in pairs {
                    let kv: Vec<&str> = pair.split('=').collect();
                    part_rating.insert(kv[0].to_string(), kv[1].parse::<u32>().unwrap());
                }
                part_ratings.push(part_rating);
            } else {
                // parse workflows
                let (key, workflow) = line.split_once('{').unwrap();
                let workflow_str = workflow.trim_matches('}');
                workflows.insert(
                    key.to_string(),
                    workflow_str.split(',').map(|s| s.to_string()).collect(),
                );
            }
        }

        // parse each line: operation: function
        Ok(Input {
            workflows,
            part_ratings,
        })
    }
}

fn part_1(input_file: &str) -> std::io::Result<u32> {
    let input: Input = parse(input_file)?;
    let mut total = 0;
    println!("workflows: {:?}", input.workflows);
    for part_rating in input.part_ratings {
        // process workflow
        let mut current_workflow = "in";
        while current_workflow != "R" && current_workflow != "A" {
            println!("current_workflow: {}", current_workflow);
            let workflow = &input.workflows[current_workflow];
            for operation in workflow {
                if operation == "R" || operation == "A" || input.workflows.contains_key(operation) {
                    current_workflow = operation;
                    break;
                }
                // This must be an operation
                let (variable, rest) = operation.split_at(1);
                let operator = if rest.starts_with('<') { "<" } else { ">" };
                let rest = rest[1..].split(":").collect::<Vec<&str>>();
                let value: u32 = rest[0].parse().unwrap();
                let value_to_compare = part_rating[variable];
                let next_workflow = rest[1];
                println!(
                    "variable {}, val: {}, next: {},  operator: {}",
                    variable, value_to_compare, next_workflow, operator
                );

                if operator == "<" && value_to_compare < value {
                    current_workflow = next_workflow;
                    break;
                } else if operator == ">" && value_to_compare > value {
                    current_workflow = next_workflow;
                    break;
                }
            }
        }

        if current_workflow == "A" {
            total += part_rating["x"] + part_rating["m"] + part_rating["a"] + part_rating["s"];
        }
    }

    Ok(total)
}

fn part_2(input_file: &str) -> std::io::Result<u32> {
    let input: Input = parse(input_file)?;
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::{part_1, part_2};

    const INPUT: &str = "input/roar/q19_input.txt";
    const INPUT_SAMPLE: &str = "input/roar/q19_sample.txt";

    #[test]
    fn roar_q19_p1_sample() {
        let result = part_1(INPUT_SAMPLE);
        assert_eq!(result.unwrap(), 19114);
    }

    #[test]
    fn roar_q19_p1_main() {
        let result = part_1(INPUT);
        assert_eq!(result.unwrap(), 331208);
    }

    #[test]
    fn roar_q19_p2_sample() {
        let result = part_2(INPUT_SAMPLE);
        assert_eq!(result.unwrap(), 0);
    }

    #[test]
    fn roar_q19_p2_main() {
        let result = part_2(INPUT);
        assert_eq!(result.unwrap(), 0);
    }
}
