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

fn reverse_graph(graph: &Graph) -> Graph {
    let mut new_graph = Graph::new();

    graph.iter().for_each(|(node, children)| {
        children.iter().for_each(|child| {
            new_graph
                .entry(child.to_string())
                .or_default()
                .push(node.to_string());
        });
    });

    new_graph
}

fn traverse_to(graph: &Graph, src: String, src_count: u64, dest: String) -> u64 {
    fn traverse(graph: &Graph, node_count: &mut HashMap<String, u64>, current_node: String) -> u64 {
        if let Some(count) = node_count.get(&current_node) {
            return *count;
        }
        let count = {
            match graph.get(&current_node) {
                Some(children) => children
                    .iter()
                    .map(|child| traverse(graph, node_count, child.to_string()))
                    .sum(),
                None => 0,
            }
        };

        node_count.insert(current_node, count);
        count
    }

    traverse(
        graph,
        &mut [(src.to_string(), src_count)].into_iter().collect(),
        dest.to_string(),
    )
}

fn p2(input: &str) -> String {
    let graph = reverse_graph(&parse_input(input));

    let svr_to_dac = traverse_to(&graph, "svr".to_string(), 1, "dac".to_string());
    let dac_to_fft = traverse_to(&graph, "dac".to_string(), svr_to_dac, "fft".to_string());
    let fft_to_out = traverse_to(&graph, "fft".to_string(), dac_to_fft, "out".to_string());

    let svr_to_fft = traverse_to(&graph, "svr".to_string(), 1, "fft".to_string());
    let fft_to_dac = traverse_to(&graph, "fft".to_string(), svr_to_fft, "dac".to_string());
    let dac_to_out = traverse_to(&graph, "dac".to_string(), fft_to_dac, "out".to_string());

    (fft_to_out + dac_to_out).to_string()
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

    #[test]
    fn test_p1_sample() {
        assert_eq!(p1(SAMPLE_INPUT_P1), "5");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "566");
    }

    const SAMPLE_INPUT_P2: &str = r"
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
";

    #[test]
    fn test_p2_sample() {
        assert_eq!(p2(SAMPLE_INPUT_P2), "2");
    }

    #[test]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "331837854931968");
    }
}
