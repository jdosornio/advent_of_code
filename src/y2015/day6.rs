use fancy_regex::Regex;
// #[derive(Eq, PartialEq, Hash, Clone)]
struct Point(usize, usize);

impl Point {
    fn from(string: &str) -> Self {
        let coords: Vec<&str> = string.split(',').collect();

        Point(coords[0].parse().unwrap(), coords[1].parse().unwrap())
    }
}

enum Operation {
    TurnOn,
    TurnOff,
    Toggle,
}

impl Operation {
    fn from(string: &str) -> Option<Self> {
        match string {
            "turn on" => Some(Self::TurnOn),
            "turn off" => Some(Self::TurnOff),
            "toggle" => Some(Self::Toggle),
            _ => None,
        }
    }
}

struct LightMatrix {
    content: Vec<Vec<bool>>,
}

impl LightMatrix {
    fn new() -> Self {
        LightMatrix {
            content: vec![vec![false; 1000]; 1000],
        }
    }

    fn mutate<F: Fn(&mut bool)>(&mut self, start: Point, end: Point, op: F) {
        for col in &mut self.content[start.0..=end.0] {
            for lit in &mut col[start.1..=end.1] {
                op(lit);
            }
        }
    }

    fn turn_on(&mut self, start: Point, end: Point) {
        self.mutate(start, end, |lit| *lit = true);
    }
    fn toggle(&mut self, start: Point, end: Point) {
        self.mutate(start, end, |lit| *lit = !(*lit));
    }
    fn turn_off(&mut self, start: Point, end: Point) {
        self.mutate(start, end, |lit| *lit = false);
    }

    fn get_lit_count(&self) -> usize {
        self.content
            .iter()
            .map(|col| col.iter().filter(|&e| *e).count())
            .sum()
    }
}

struct BrightnessMatrix {
    content: Vec<Vec<usize>>,
}

impl BrightnessMatrix {
    fn new() -> Self {
        BrightnessMatrix {
            content: vec![vec![0; 1000]; 1000],
        }
    }

    fn mutate<F: Fn(&mut usize)>(&mut self, start: Point, end: Point, op: F) {
        for col in &mut self.content[start.0..=end.0] {
            for lit in &mut col[start.1..=end.1] {
                op(lit);
            }
        }
    }

    fn turn_on(&mut self, start: Point, end: Point) {
        self.mutate(start, end, |lit| *lit += 1);
    }
    fn toggle(&mut self, start: Point, end: Point) {
        self.mutate(start, end, |lit| *lit += 2);
    }
    fn turn_off(&mut self, start: Point, end: Point) {
        self.mutate(start, end, |lit| *lit = if *lit > 0 { *lit - 1 } else { 0 });
    }

    fn get_lit_count(&self) -> usize {
        self.content
            .iter()
            .map(|col| -> usize { col.iter().sum() })
            .sum()
    }
}

pub fn puzzle1(input: &str) -> usize {
    let mut matrix = LightMatrix::new();
    let regex_line = Regex::new(r"^(?<op>[a-z\s]+)\s(?<start>\d+,\d+).*?(?<end>\d+,\d+)$").unwrap();

    input.lines().for_each(|line| {
        let result = regex_line.captures(line).unwrap().unwrap();
        let op = Operation::from(result.name("op").unwrap().as_str()).unwrap();
        let start = Point::from(result.name("start").unwrap().as_str());
        let end = Point::from(result.name("end").unwrap().as_str());

        match op {
            Operation::TurnOn => matrix.turn_on(start, end),
            Operation::TurnOff => matrix.turn_off(start, end),
            Operation::Toggle => matrix.toggle(start, end),
        }
    });

    matrix.get_lit_count()
}

pub fn puzzle2(input: &str) -> usize {
    let mut matrix = BrightnessMatrix::new();
    let regex_line = Regex::new(r"^(?<op>[a-z\s]+)\s(?<start>\d+,\d+).*?(?<end>\d+,\d+)$").unwrap();

    input.lines().for_each(|line| {
        let result = regex_line.captures(line).unwrap().unwrap();
        let op = Operation::from(result.name("op").unwrap().as_str()).unwrap();
        let start = Point::from(result.name("start").unwrap().as_str());
        let end = Point::from(result.name("end").unwrap().as_str());

        match op {
            Operation::TurnOn => matrix.turn_on(start, end),
            Operation::TurnOff => matrix.turn_off(start, end),
            Operation::Toggle => matrix.toggle(start, end),
        }
    });

    matrix.get_lit_count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn puzzle1_test() {
        let tests = [
            ("turn on 0,0 through 999,999", 1_000_000),
            ("toggle 0,0 through 999,0", 1_000),
            ("turn off 499,499 through 500,500", 0),
        ];

        for (i, (input, result)) in tests.iter().enumerate() {
            assert_eq!(&puzzle1(input), result, "Test {} failed", i);
        }
    }

    #[test]
    fn puzzle2_test() {
        let tests = [
            ("turn on 0,0 through 0,0", 1),
            ("toggle 0,0 through 999,999", 2_000_000),
        ];

        for (i, (input, result)) in tests.iter().enumerate() {
            assert_eq!(&puzzle2(input), result, "Test {} failed", i);
        }
    }
}
