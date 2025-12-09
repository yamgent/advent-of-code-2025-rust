const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2025/09/input.txt");

fn parse_input(input: &str) -> Vec<(i64, i64)> {
    input
        .trim()
        .lines()
        .map(|line| line.split_once(",").expect("x,x"))
        .map(|coord| {
            (
                coord.0.parse().expect("a number"),
                coord.1.parse().expect("a number"),
            )
        })
        .collect()
}

fn area(a: &(i64, i64), b: &(i64, i64)) -> i64 {
    ((a.0 - b.0).abs() + 1) * ((a.1 - b.1).abs() + 1)
}

fn p1(input: &str) -> String {
    let coords = parse_input(input);
    coords
        .iter()
        .map(|coord| {
            coords
                .iter()
                .map(|other| area(coord, other))
                .max()
                .expect("always have at least one area")
        })
        .max()
        .expect("always have an answer")
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

    const SAMPLE_INPUT: &str = r"
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";

    #[test]
    fn test_p1_sample() {
        assert_eq!(p1(SAMPLE_INPUT), "50");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "4741848414");
    }

    #[test]
    fn test_p2_sample() {
        assert_eq!(p2(SAMPLE_INPUT), "");
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "");
    }
}
