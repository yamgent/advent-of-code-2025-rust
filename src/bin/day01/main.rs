const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2025/01/input.txt");

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Instruction {
    Left(u32),
    Right(u32),
}

impl Instruction {
    fn parse(line: &str) -> Self {
        let direction = line.chars().next().expect("a character");
        let amount = line
            .chars()
            .skip(1)
            .collect::<String>()
            .parse()
            .expect("a number");

        match direction {
            'L' => Self::Left(amount),
            'R' => Self::Right(amount),
            _ => {
                panic!("expect L or R");
            }
        }
    }
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input.trim().lines().map(Instruction::parse).collect()
}

fn p1(input: &str) -> String {
    parse_input(input)
        .into_iter()
        .fold((50i32, 0u32), |mut acc, line| {
            match line {
                Instruction::Left(amount) => {
                    acc.0 -= amount as i32;
                }
                Instruction::Right(amount) => {
                    acc.0 += amount as i32;
                }
            }

            acc.0 %= 100;
            if acc.0 == 0 {
                acc.1 += 1;
            }

            acc
        })
        .1
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

    #[test]
    fn test_instructions() {
        assert_eq!(
            parse_input(
                r"
L20
R30
"
            ),
            vec![Instruction::Left(20), Instruction::Right(30)]
        );
    }

    const SAMPLE_INPUT: &str = r"
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";

    #[test]
    fn test_p1_sample() {
        assert_eq!(p1(SAMPLE_INPUT), "3");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "1066");
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
