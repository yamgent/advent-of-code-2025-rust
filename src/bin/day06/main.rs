const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2025/06/input.txt");

enum Equation {
    Add(Vec<i64>),
    Mul(Vec<i64>),
}

impl Equation {
    fn calc(&self) -> i64 {
        match self {
            Self::Add(nums) => nums.iter().sum::<i64>(),
            Self::Mul(nums) => nums.iter().product::<i64>(),
        }
    }
}

fn parse_input(input: &str) -> Vec<Equation> {
    let numbers = input
        .trim()
        .lines()
        .rev()
        .skip(1)
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<i64>().expect("a number"))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    input
        .trim()
        .lines()
        .rev()
        .next()
        .expect("at least two lines, first reversed line contains the symbols")
        .split_whitespace()
        .enumerate()
        .map(|(column_idx, symbol)| {
            let column_numbers = numbers
                .iter()
                .map(|row| row[column_idx])
                .collect::<Vec<_>>();

            if symbol == "+" {
                Equation::Add(column_numbers)
            } else if symbol == "*" {
                Equation::Mul(column_numbers)
            } else {
                panic!("expect + or *, found {}", symbol);
            }
        })
        .collect()
}

fn p1(input: &str) -> String {
    parse_input(input)
        .into_iter()
        .map(|entry| entry.calc())
        .sum::<i64>()
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
123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +
";

    #[test]
    fn test_p1_sample() {
        assert_eq!(p1(SAMPLE_INPUT), "4277556");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "4722948564882");
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
