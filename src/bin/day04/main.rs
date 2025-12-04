use std::collections::HashSet;

const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2025/04/input.txt");

fn p1(input: &str) -> String {
    let grid: HashSet<_> =
        HashSet::from_iter(input.trim().lines().enumerate().flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, ch)| *ch == '@')
                .map(move |(x, _)| (x as i64, y as i64))
        }));

    let size = (
        input
            .trim()
            .lines()
            .next()
            .expect("at least one row")
            .chars()
            .count(),
        input.trim().lines().count(),
    );

    (0_i64..size.0 as i64)
        .map(|x| {
            (0_i64..size.1 as i64)
                .filter(|y| grid.contains(&(x, *y)))
                .filter(|y| {
                    let coord = (x, *y);

                    [
                        (-1, -1),
                        (-1, 0),
                        (-1, 1),
                        (0, -1),
                        (0, 1),
                        (1, -1),
                        (1, 0),
                        (1, 1),
                    ]
                    .into_iter()
                    .map(|relative| (coord.0 + relative.0, coord.1 + relative.1))
                    .filter(|adjacent| grid.contains(adjacent))
                    .count()
                        < 4
                })
                .count()
        })
        .sum::<usize>()
        .to_string()
}

fn p2(input: &str) -> String {
    let mut grid: HashSet<_> =
        HashSet::from_iter(input.trim().lines().enumerate().flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, ch)| *ch == '@')
                .map(move |(x, _)| (x as i64, y as i64))
        }));

    let size = (
        input
            .trim()
            .lines()
            .next()
            .expect("at least one row")
            .chars()
            .count(),
        input.trim().lines().count(),
    );

    let mut iterations = 0;
    let mut count = 0;
    let mut last_coords = vec![];

    while iterations == 0 || !last_coords.is_empty() {
        // not the most efficient but at least it does the job now
        last_coords = (0_i64..size.0 as i64)
            .flat_map(|x| {
                (0_i64..size.1 as i64)
                    .filter(|y| grid.contains(&(x, *y)))
                    .filter(|y| {
                        let coord = (x, *y);

                        [
                            (-1, -1),
                            (-1, 0),
                            (-1, 1),
                            (0, -1),
                            (0, 1),
                            (1, -1),
                            (1, 0),
                            (1, 1),
                        ]
                        .into_iter()
                        .map(|relative| (coord.0 + relative.0, coord.1 + relative.1))
                        .filter(|adjacent| grid.contains(adjacent))
                        .count()
                            < 4
                    })
                    .map(|y| (x, y))
                    .collect::<Vec<_>>()
            })
            .collect();

        iterations += 1;
        count += last_coords.len();

        last_coords.iter().for_each(|coord| {
            grid.remove(coord);
        });
    }

    count.to_string()
}

fn main() {
    println!("{}", p1(ACTUAL_INPUT));
    println!("{}", p2(ACTUAL_INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = r"
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";

    #[test]
    fn test_p1_sample() {
        assert_eq!(p1(SAMPLE_INPUT), "13");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "1474");
    }

    #[test]
    fn test_p2_sample() {
        assert_eq!(p2(SAMPLE_INPUT), "43");
    }

    #[test]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "8910");
    }
}
