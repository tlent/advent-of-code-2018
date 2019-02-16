use std::collections::HashSet;
use std::hash::Hash;

const INPUT: &str = include_str!("../input");
const WORKERS: usize = 5;
const BASE_TIME: u32 = 60;
const FIRST_STEP_INDEX: usize = 5;
const SECOND_STEP_INDEX: usize = 36;

fn main() {
    let parsed_input = parse_input(INPUT);
    println!("{}", solve_part_one(&parsed_input));
    println!("{}", solve_part_two(&parsed_input, WORKERS, BASE_TIME));
}

#[derive(Debug, PartialEq)]
struct Graph<T>
where
    T: Eq + Hash,
{
    nodes: HashSet<T>,
    edges: Vec<(T, T)>,
}

impl<T> Graph<T>
where
    T: Eq + Hash + Clone,
{
    fn from_edges(edges: &[(T, T)]) -> Self {
        let nodes = edges.iter().fold(HashSet::new(), |mut acc, (a, b)| {
            acc.insert(a.clone());
            acc.insert(b.clone());
            acc
        });
        Graph {
            nodes,
            edges: edges.to_vec(),
        }
    }
}

fn parse_input(input: &str) -> Graph<char> {
    let edges: Vec<(char, char)> = input
        .trim()
        .split('\n')
        .map(|line| {
            (
                line.chars().nth(FIRST_STEP_INDEX).unwrap(),
                line.chars().nth(SECOND_STEP_INDEX).unwrap(),
            )
        })
        .collect();
    Graph::from_edges(&edges)
}

// Kahn's algorithm for topological sort
fn solve_part_one(g: &Graph<char>) -> String {
    let steps = &g.nodes;
    let mut result = String::new();
    let mut remaining_requirements = g.edges.clone();
    let mut ready_steps: Vec<_> = steps
        .iter()
        // Filter out steps that depend on any other step (these are not ready to complete)
        .filter(|&&step| {
            !remaining_requirements
                .iter()
                .any(|&(_, dependent)| dependent == step)
        })
        .collect();
    while !ready_steps.is_empty() {
        // Complete the first (alphabetically ordered) ready step
        let completed_step = **ready_steps.iter().min().unwrap();
        result.push(completed_step);

        remaining_requirements = remaining_requirements
            .into_iter()
            // Filter out requirements that depend on the completed step (these have been fulfilled)
            .filter(|&(depended, _)| depended != completed_step)
            .collect();

        ready_steps = steps
            .iter()
            .filter(|&&step| {
                // Filter out steps that have already been completed
                !result.chars().any(|c| c == step)
                // Filter out steps that depend on any other step (these are not ready to complete)
                && !remaining_requirements.iter().any(|&(_, dependent)| dependent == step)
            })
            .collect();
    }
    if !remaining_requirements.is_empty() {
        panic!("cycle detected");
    }
    result
}

fn solve_part_two(g: &Graph<char>, worker_count: usize, base_time: u32) -> u32 {
    let steps = &g.nodes;
    let mut time_passed = 0;
    let mut workers: Vec<Option<(char, u32)>> = vec![None; worker_count];
    let mut completed_steps = HashSet::new();
    let mut remaining_requirements = g.edges.clone();
    let mut ready_steps: Vec<_> = steps
        .iter()
        .filter(|&&step| {
            // Filter out steps that depend on any other step (these are not ready to complete)
            !remaining_requirements
                .iter()
                .any(|&(_, dependent)| dependent == step)
        })
        .collect();
    while !ready_steps.is_empty() || workers.iter().filter(|w| w.is_some()).count() != 0 {
        let iter = workers
            .iter_mut()
            .filter(|w| w.is_none())
            .zip(ready_steps.iter());
        for (ready_worker, &ready_step) in iter {
            *ready_worker = Some((
                *ready_step,
                u32::from(*ready_step as u8 - b'A' + 1) + base_time,
            ));
        }
        let t = workers
            .iter()
            .filter_map(|w| match w {
                Some((_, t)) => Some(*t),
                None => None,
            })
            .min()
            .unwrap();
        let finished_workers = workers
            .iter_mut()
            .filter(|w| w.is_some() && w.unwrap().1 == t);
        for finished_worker in finished_workers {
            let completed_step = finished_worker.unwrap().0;
            completed_steps.insert(completed_step);
            *finished_worker = None;
            // Filter out requirements that depend on the completed step (these have been fulfilled)
            remaining_requirements.retain(|&(depended, _)| depended != completed_step);
        }
        time_passed += t;
        workers = workers
            .iter()
            .map(|w| match w {
                Some(w) => Some((w.0, w.1 - t)),
                None => None,
            })
            .collect();

        ready_steps = steps
            .iter()
            .filter(|&&step| {
                // Filter out steps that have already been completed
                !completed_steps.contains(&step)
                // Filter out steps that are already being worked on
                && !workers.iter().any(|w| w.is_some() && w.unwrap().0 == step)
                // Filter out steps that depend on any other step (these are not ready to complete)
                && !remaining_requirements.iter().any(|&(_, dependent)| dependent == step)
            })
            .collect();
        ready_steps.sort();
    }
    if !remaining_requirements.is_empty() {
        panic!("cycle detected");
    }
    time_passed
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE_INPUT_STR: &str = include_str!("../sample-input");

    #[test]
    fn it_parses_input_correctly() {
        let sample_graph = get_sample_graph();
        assert_eq!(parse_input(SAMPLE_INPUT_STR), sample_graph);
    }

    #[test]
    fn it_solves_part_one_correctly() {
        assert_eq!(solve_part_one(&get_sample_graph()), "CABDFE");
    }

    #[test]
    fn it_solves_part_two_correctly() {
        assert_eq!(solve_part_two(&get_sample_graph(), 2, 0), 15);
    }

    fn get_sample_graph() -> Graph<char> {
        Graph::from_edges(&[
            ('C', 'A'),
            ('C', 'F'),
            ('A', 'B'),
            ('A', 'D'),
            ('B', 'E'),
            ('D', 'E'),
            ('F', 'E'),
        ])
    }
}
