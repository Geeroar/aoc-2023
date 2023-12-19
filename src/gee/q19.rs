#![allow(dead_code, unused_variables)]

use crate::utils::parser::{parse, FileLines};
use std::cmp::{max, min};
use std::collections::{HashMap, VecDeque};

type Constraint = (
    (usize, usize),
    (usize, usize),
    (usize, usize),
    (usize, usize),
);

#[derive(Debug)]
struct Input {
    workflows: Vec<String>,
    workflow_map: HashMap<String, Vec<Rule>>,
    parts: Vec<Part>,
}

#[derive(Debug)]
enum Rule {
    Condition(char, char, usize, String),
    Fallback(String),
}

#[derive(Debug)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl TryFrom<FileLines> for Input {
    type Error = std::io::Error;

    fn try_from(mut lines: FileLines) -> Result<Self, Self::Error> {
        let mut workflows = Vec::new();
        let mut workflow_map = HashMap::new();
        let mut parts = Vec::new();
        for line in lines.by_ref() {
            if line.is_empty() {
                break;
            }
            let (name, r) = line.strip_suffix('}').unwrap().split_once('{').unwrap();
            let rules = r
                .split(',')
                .map(|s| {
                    if s.contains(':') {
                        let var = s.chars().next().unwrap();
                        let op = s.chars().nth(1).unwrap();
                        let (val, dest) = s[2..].split_once(':').unwrap();
                        Rule::Condition(var, op, val.parse().unwrap(), String::from(dest))
                    } else {
                        Rule::Fallback(String::from(s))
                    }
                })
                .collect();
            workflows.push(String::from(name));
            workflow_map.insert(String::from(name), rules);
        }
        for line in lines {
            let vars = line
                .strip_prefix('{')
                .unwrap()
                .strip_suffix('}')
                .unwrap()
                .split(',')
                .collect::<Vec<_>>();
            parts.push(Part {
                x: vars[0].split_once('=').unwrap().1.parse().unwrap(),
                m: vars[1].split_once('=').unwrap().1.parse().unwrap(),
                a: vars[2].split_once('=').unwrap().1.parse().unwrap(),
                s: vars[3].split_once('=').unwrap().1.parse().unwrap(),
            });
        }
        Ok(Input {
            workflows,
            workflow_map,
            parts,
        })
    }
}

impl Input {
    fn total_rating(&self) -> usize {
        self.parts
            .iter()
            .map(|p| {
                if self.is_accepted(p) {
                    p.x + p.m + p.a + p.s
                } else {
                    0
                }
            })
            .sum()
    }

    fn is_accepted(&self, part: &Part) -> bool {
        let mut rules = self.workflow_map.get("in").unwrap().iter();
        while let Some(rule) = rules.next() {
            match rule {
                Rule::Condition(var, op, val, dest) if dest.as_str() == "A" => {
                    if self.passes_rule(part, *var, *op, *val) {
                        return true;
                    }
                }
                Rule::Condition(var, op, val, dest) if dest.as_str() == "R" => {
                    if self.passes_rule(part, *var, *op, *val) {
                        return false;
                    }
                }
                Rule::Condition(var, op, val, dest) => {
                    if self.passes_rule(part, *var, *op, *val) {
                        rules = self.workflow_map.get(dest).unwrap().iter();
                    }
                }
                Rule::Fallback(dest) if dest.as_str() == "A" => return true,
                Rule::Fallback(dest) if dest.as_str() == "R" => return false,
                Rule::Fallback(dest) => {
                    rules = self.workflow_map.get(dest).unwrap().iter();
                }
            }
        }
        false
    }

    fn passes_rule(&self, part: &Part, var: char, op: char, val: usize) -> bool {
        let v = match var {
            'x' => part.x,
            'm' => part.m,
            'a' => part.a,
            's' => part.s,
            _ => panic!("Bad variable name"),
        };
        (op == '>' && v > val) || (op == '<' && v < val)
    }
}

fn apply_constraint(constraint: Constraint, var: char, op: char, val: usize) -> Constraint {
    let mut c = constraint;
    let v = match var {
        'x' => &mut c.0,
        'm' => &mut c.1,
        'a' => &mut c.2,
        's' => &mut c.3,
        _ => panic!("Bad constraint var"),
    };
    match op {
        '>' => v.0 = val + 1,
        '<' => v.1 = val - 1,
        _ => panic!("Bad constraint op"),
    }
    c
}

fn negative_constraint(constraint: Constraint, var: char, op: char, val: usize) -> Constraint {
    match op {
        '>' => apply_constraint(constraint, var, '<', val + 1),
        '<' => apply_constraint(constraint, var, '>', val - 1),
        _ => panic!("Bad constraint op"),
    }
}

fn combine_constraints(c1: Constraint, c2: Constraint) -> Constraint {
    let ((x1l, x1u), (m1l, m1u), (a1l, a1u), (s1l, s1u)) = c1;
    let ((x2l, x2u), (m2l, m2u), (a2l, a2u), (s2l, s2u)) = c2;
    let x = (max(x1l, x2l), min(x1u, x2u));
    let m = (max(m1l, m2l), min(m1u, m2u));
    let a = (max(a1l, a2l), min(a1u, a2u));
    let s = (max(s1l, s2l), min(s1u, s2u));
    (x, m, a, s)
}

fn constraint_combinations(c: Constraint) -> usize {
    let ((xl, xu), (ml, mu), (al, au), (sl, su)) = c;
    (xu + 1 - xl) * (mu + 1 - ml) * (au + 1 - al) * (su + 1 - sl)
}

fn starting_constraint(min: usize, max: usize) -> Constraint {
    ((min, max), (min, max), (min, max), (min, max))
}

fn part_1(input_file: &str) -> std::io::Result<usize> {
    let input: Input = parse(input_file)?;
    Ok(input.total_rating())
}

fn part_2(input_file: &str) -> std::io::Result<usize> {
    let input: Input = parse(input_file)?;
    let mut queue = VecDeque::<(Constraint, String)>::new();
    let mut accepted = Vec::<Constraint>::new();
    queue.push_back((starting_constraint(1, 4000), String::from("in")));
    while let Some((mut constraint, workflow)) = queue.pop_front() {
        let rules = input.workflow_map.get(&workflow).unwrap();
        for rule in rules {
            match rule {
                Rule::Condition(var, op, val, dest) if dest.as_str() == "A" => {
                    accepted.push(apply_constraint(constraint, *var, *op, *val));
                    constraint = negative_constraint(constraint, *var, *op, *val);
                }
                Rule::Condition(var, op, val, dest) if dest.as_str() == "R" => {
                    constraint = negative_constraint(constraint, *var, *op, *val);
                }
                Rule::Condition(var, op, val, dest) => {
                    queue.push_back((apply_constraint(constraint, *var, *op, *val), dest.clone()));
                    constraint = negative_constraint(constraint, *var, *op, *val);
                }
                Rule::Fallback(dest) if dest.as_str() == "A" => {
                    accepted.push(constraint);
                }
                Rule::Fallback(dest) if dest.as_str() == "R" => (),
                Rule::Fallback(dest) => {
                    queue.push_back((constraint, dest.clone()));
                }
            }
        }
    }
    let total = accepted.iter().map(|c| constraint_combinations(*c)).sum();
    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::{apply_constraint, combine_constraints, part_1, part_2};

    const INPUT: &str = "input/gee/q19_input.txt";
    const INPUT_SAMPLE: &str = "input/gee/q19_sample.txt";

    #[test]
    fn gee_q19_testio() {
        let c1 = ((1, 4000), (1, 4000), (1, 4000), (1, 4000));
        let c2 = apply_constraint(c1, 'm', '<', 35);
        let c3 = apply_constraint(c1, 'x', '>', 3032);
        let c4 = combine_constraints(c2, c3);
        assert_eq!(c4, ((3033, 4000), (1, 34), (1, 4000), (1, 4000)));
    }

    #[test]
    fn gee_q19_p1_sample() {
        let result = part_1(INPUT_SAMPLE);
        assert_eq!(result.unwrap(), 19114);
    }

    #[test]
    fn gee_q19_p1_main() {
        let result = part_1(INPUT);
        assert_eq!(result.unwrap(), 472630);
    }

    #[test]
    fn gee_q19_p2_sample() {
        let result = part_2(INPUT_SAMPLE);
        assert_eq!(result.unwrap(), 167409079868000);
    }

    #[test]
    fn gee_q19_p2_main() {
        let result = part_2(INPUT);
        assert_eq!(result.unwrap(), 116738260946855);
    }
}
