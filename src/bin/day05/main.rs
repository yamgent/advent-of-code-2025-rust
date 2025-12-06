const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2025/05/input.txt");

struct Input {
    ranges: Vec<(i64, i64)>,
    ingredients: Vec<i64>,
}

fn parse_input(input: &str) -> Input {
    let (ranges, ingredients) = input
        .trim()
        .split_once("\n\n")
        .expect("two sections of input");

    let ranges = ranges
        .trim()
        .lines()
        .map(|line| line.split_once("-").expect("x-x"))
        .map(|(start, end)| {
            (
                start.parse::<i64>().expect("a number"),
                end.parse::<i64>().expect("a number"),
            )
        })
        .collect::<Vec<_>>();

    let ingredients = ingredients
        .trim()
        .lines()
        .map(|line| line.parse::<i64>().expect("a number"))
        .collect::<Vec<_>>();

    Input {
        ranges,
        ingredients,
    }
}

fn in_range(range: &(i64, i64), ingredient: i64) -> bool {
    range.0 <= ingredient && range.1 >= ingredient
}

fn p1(input: &str) -> String {
    let input = parse_input(input);
    input
        .ingredients
        .iter()
        .filter(|ingredient| {
            input
                .ranges
                .iter()
                .any(|range| in_range(range, **ingredient))
        })
        .count()
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
3-5
10-14
16-20
12-18

1
5
8
11
17
32
";

    #[test]
    fn test_p1_sample() {
        assert_eq!(p1(SAMPLE_INPUT), "3");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "712");
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
