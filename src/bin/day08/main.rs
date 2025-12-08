use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

use union_find::{QuickFindUf, UnionBySize, UnionFind};

const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2025/08/input.txt");

type Coord = (i64, i64, i64);

fn parse_input(input: &str) -> Vec<Coord> {
    input
        .trim()
        .lines()
        .map(|line| {
            let mut iter = line.split(",").map(|x| x.parse().expect("a number"));

            (
                iter.next().expect("3 entries"),
                iter.next().expect("3 entries"),
                iter.next().expect("3 entries"),
            )
        })
        .collect()
}

fn dist_squared(a: &Coord, b: &Coord) -> i64 {
    (b.0 - a.0).pow(2) + (b.1 - a.1).pow(2) + (b.2 - a.2).pow(2)
}

fn solve_p1(input: &str, connections: usize) -> i64 {
    let input = parse_input(input);

    let mut dists = input
        .iter()
        .enumerate()
        .flat_map(|(current_idx, current)| {
            input
                .iter()
                .enumerate()
                .skip(current_idx + 1)
                .map(move |(entry_idx, entry)| {
                    (
                        Reverse(dist_squared(current, entry)),
                        current_idx,
                        entry_idx,
                    )
                })
        })
        .collect::<BinaryHeap<_>>();

    let mut ufs = QuickFindUf::<UnionBySize>::new(input.len());

    (0..connections).for_each(|_| {
        let next = dists.pop().expect("still have a candidate");

        ufs.union(next.1, next.2);
    });

    let mut counters = HashMap::new();

    (0..input.len()).for_each(|idx| {
        let collection = ufs.find(idx);
        *counters.entry(collection).or_insert(0) += 1;
    });

    let mut counters = counters
        .into_iter()
        .map(|(collection, count)| (Reverse(count), collection))
        .collect::<Vec<_>>();
    counters.sort_unstable();

    counters
        .into_iter()
        .take(3)
        .map(|(count, _)| count.0)
        .product()
}

fn p1(input: &str) -> String {
    solve_p1(input, 1000).to_string()
}

fn p2(input: &str) -> String {
    let input = parse_input(input);

    let mut dists = input
        .iter()
        .enumerate()
        .flat_map(|(current_idx, current)| {
            input
                .iter()
                .enumerate()
                .skip(current_idx + 1)
                .map(move |(entry_idx, entry)| {
                    (
                        Reverse(dist_squared(current, entry)),
                        current_idx,
                        entry_idx,
                    )
                })
        })
        .collect::<BinaryHeap<_>>();

    let mut ufs = QuickFindUf::<UnionBySize>::new(input.len());
    let mut used = HashSet::new();

    loop {
        let next = dists
            .pop()
            .expect("puzzle should have a solution before we exhaust everything");
        used.insert(next.1);
        used.insert(next.2);

        ufs.union(next.1, next.2);

        if used.len() == input.len() {
            return (input[next.1].0 * input[next.2].0).to_string();
        }
    }
}

fn main() {
    println!("{}", p1(ACTUAL_INPUT));
    println!("{}", p2(ACTUAL_INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = r"
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";

    #[test]
    fn test_p1_sample() {
        assert_eq!(solve_p1(SAMPLE_INPUT, 10), 40);
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "24360");
    }

    #[test]
    fn test_p2_sample() {
        assert_eq!(p2(SAMPLE_INPUT), "25272");
    }

    #[test]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "2185817796");
    }
}
