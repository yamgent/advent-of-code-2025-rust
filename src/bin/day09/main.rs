const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2025/09/input.txt");

type Point = (i64, i64);
type Line = (Point, Point);

fn parse_input(input: &str) -> Vec<Point> {
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

fn area(a: &Point, b: &Point) -> i64 {
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

#[derive(Debug)]
struct Rect {
    a: Point,
    b: Point,
}

impl Rect {
    fn new(a: &Point, b: &Point) -> Self {
        Self { a: *a, b: *b }
    }

    fn has_line(&self, line: &Line) -> bool {
        let left = self.a.0.min(self.b.0);
        let right = self.a.0.max(self.b.0);
        let top = self.a.1.min(self.b.1);
        let bottom = self.a.1.max(self.b.1);

        if line.0.0 == line.1.0 {
            let line_x = line.0.0;
            let line_top = line.0.1.min(line.1.1);
            let line_bottom = line.0.1.max(line.1.1);
            !(line_x <= left || line_x >= right || line_bottom < top || line_top > bottom)
        } else if line.0.1 == line.1.1 {
            let line_y = line.0.1;
            let line_left = line.0.0.min(line.1.0);
            let line_right = line.0.0.max(line.1.0);

            !(line_y <= top || line_y >= bottom || line_right < left || line_left > right)
        } else {
            panic!("Cannot handle slanted lines");
        }
    }

    fn area(&self) -> i64 {
        area(&self.a, &self.b)
    }
}

fn lines(coords: &[Point]) -> Vec<Line> {
    coords
        .iter()
        .zip(coords.iter().skip(1).chain(coords.iter().take(1)))
        .map(|(a, b)| (*a, *b))
        .collect()
}

fn p2(input: &str) -> String {
    let coords = parse_input(input);
    let lines = lines(&coords);

    dbg!(
        coords
            .iter()
            .flat_map(|coord| {
                coords
                    .iter()
                    .filter(|other| *other != coord)
                    .map(|other| Rect::new(coord, other))
                    .filter(|rect| lines.iter().all(|line| !rect.has_line(line)))
                    // .map(|rect| rect.area())
                    .max_by(|x, y| x.area().cmp(&y.area()))
                // .max()
            })
            // .max()
            .max_by(|x, y| x.area().cmp(&y.area()))
            .expect("always have an answer")
    )
    .area()
    .to_string()
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
        assert_eq!(p2(SAMPLE_INPUT), "24");
    }

    #[test]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "1508918480");
    }
}
