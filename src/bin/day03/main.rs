const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2025/03/input.txt");

fn p1(input: &str) -> String {
    input
        .trim()
        .lines()
        .map(|line| {
            let digits = line
                .trim()
                .chars()
                .map(|ch| ch.to_digit(10).expect("a digit"))
                .collect::<Vec<_>>();

            let maxs_reversed: Vec<u32> = digits.iter().rev().fold(vec![], |mut acc, entry| {
                acc.push(*acc.last().unwrap_or(entry).max(entry));
                acc
            });

            digits
                .iter()
                .zip(maxs_reversed.iter().rev().skip(1))
                .map(|(a, b)| a * 10 + b)
                .max()
                .expect("not empty array")
        })
        .sum::<u32>()
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
987654321111111
811111111111119
234234234234278
818181911112111
";

    #[test]
    fn test_p1_sample() {
        assert_eq!(p1(SAMPLE_INPUT), "357");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "17376");
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
