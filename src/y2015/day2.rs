pub fn puzzle1(input: &str) -> u32 {
    input
        .lines()
        .fold(0, |total, line| total + paper_area(line))
}

pub fn puzzle2(input: &str) -> u32 {
    input
        .lines()
        .fold(0, |total, line| total + needed_ribbon(line))
}

fn paper_area(input: &str) -> u32 {
    let sides: Vec<u32> = input.split('x').map(|num| num.parse().unwrap()).collect();

    let areas = [
        sides[0] * sides[1],
        sides[0] * sides[2],
        sides[1] * sides[2],
    ];

    areas.iter().map(|area| 2 * area).sum::<u32>() + areas.iter().min().unwrap()
}

fn needed_ribbon(input: &str) -> u32 {
    let sides: Vec<u32> = input.split('x').map(|num| num.parse().unwrap()).collect();

    let perimeters = [
        2 * sides[0] + 2 * sides[1],
        2 * sides[0] + 2 * sides[2],
        2 * sides[1] + 2 * sides[2],
    ];

    sides.iter().product::<u32>() + perimeters.iter().min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn puzzle1_test() {
        let tests = [("2x3x4", 58), ("1x1x10", 43)];

        for (i, (input, result)) in tests.iter().enumerate() {
            assert_eq!(&puzzle1(input), result, "Test {} failed", i);
        }
    }

    #[test]
    fn puzzle2_test() {
        let tests = [("2x3x4", 34), ("1x1x10", 14)];

        for (i, (input, result)) in tests.iter().enumerate() {
            assert_eq!(&puzzle2(input), result, "Test {} failed", i);
        }
    }
}
