pub fn puzzle1(input: &str) -> i32 {
    input
        .chars()
        .fold(0, |acc, elem| if elem == '(' { acc + 1 } else { acc - 1 })
}

pub fn puzzle2(input: &str) -> usize {
    let mut acc = 0;

    for (i, c) in input.chars().enumerate() {
        if c == '(' {
            acc += 1;
        } else {
            acc -= 1;
        }

        if acc < 0 {
            return i + 1;
        }
    }

    //Should be -1 instead?
    input.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn puzzle1_test() {
        let tests = [
            ("(())", 0),
            ("()()", 0),
            ("(((", 3),
            ("(()(()(", 3),
            ("))(((((", 3),
            ("())", -1),
            ("))(", -1),
            (")))", -3),
            (")())())", -3),
        ];

        for (i, (input, result)) in tests.iter().enumerate() {
            assert_eq!(&puzzle1(input), result, "Test {} failed", i);
        }
    }

    #[test]
    fn puzzle2_test() {
        let tests = [(")", 1), ("()())", 5)];

        for (i, (input, result)) in tests.iter().enumerate() {
            assert_eq!(&puzzle2(input), result, "Test {} failed", i);
        }
    }
}
