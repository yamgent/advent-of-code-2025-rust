use std::collections::{HashMap, HashSet};

const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2025/11/input.txt");

type Graph = HashMap<String, Vec<String>>;

fn parse_input(input: &str) -> Graph {
    input
        .trim()
        .lines()
        .map(|line| {
            let (current, children) = line.split_once(": ").expect("xx: xx xx");
            (
                current.to_string(),
                children.split(" ").map(|child| child.to_string()).collect(),
            )
        })
        .collect()
}

fn toposort(graph: &Graph, src: String, dest: String) -> Vec<String> {
    fn traverse(
        graph: &Graph,
        dest: String,
        result: &mut Vec<String>,
        visited: &mut HashSet<String>,
        current: String,
    ) {
        visited.insert(current.clone());

        if current != dest
            && let Some(children) = graph.get(&current)
        {
            children.iter().for_each(|child| {
                if visited.contains(child) {
                    return;
                }

                traverse(graph, dest.clone(), result, visited, child.clone());
            });
        }

        result.push(current);
    }

    let mut result = vec![];
    traverse(graph, dest.clone(), &mut result, &mut HashSet::new(), src);
    result.reverse();
    result
}

fn p1(input: &str) -> String {
    let graph = parse_input(input);
    let order = toposort(&graph, "you".to_string(), "out".to_string());
    let mut counts = [("you".to_string(), 1)]
        .into_iter()
        .collect::<HashMap<_, _>>();

    order.iter().for_each(|node| {
        let count = *counts
            .get(node)
            .expect("because of toposort, we should have visited this before");
        if let Some(children) = graph.get(node) {
            children.iter().for_each(|child| {
                *counts.entry(child.clone()).or_default() += count;
            });
        }
    });

    counts
        .get("out")
        .expect("out should have been visited")
        .to_string()
}

fn p2(input: &str) -> String {
    let _input = input.trim();
    "".to_string()
}

fn main() {
    println!("{}", p1(ACTUAL_INPUT));
    println!("{}", p2(ACTUAL_INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT_P1: &str = r"
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
";

    const SAMPLE_INPUT_P2: &str = r"";

    #[test]
    fn test_p1_sample() {
        assert_eq!(p1(SAMPLE_INPUT_P1), "5");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "566");
    }

    #[test]
    fn test_p2_sample() {
        assert_eq!(p2(SAMPLE_INPUT_P2), "");
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "");
    }
}
