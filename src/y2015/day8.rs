use fancy_regex::Regex;

pub fn puzzle1(input: &str) -> usize {
    // Not the fanciest
    let re = Regex::new("\\\\\\\\|\\\\\"|\\\\x[0-9a-f]{2}").unwrap();

    let total = input.lines().fold(0, |sum, line| {
        let orig_count = line.len();
        let line = re.replace_all(line, "1");

        sum + (orig_count - (line.len() - 2))
    });

    total
}

pub fn puzzle2(input: &str) -> usize {
    //Not the most optimal approach
    let re = Regex::new("\"|\\\\").unwrap();

    input.lines().fold(0, |sum, line| {
        let count_before = line.len();
        let line = re.replace_all(line, "11");

        sum + (line.len() + 2 - count_before)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn puzzle1_test() {
        let tests = [
            ("\"\"", 2),
            ("\"abc\"", 2),
            ("\"aaa\\\"aaa\"", 3),
            ("\"\\x27\"", 5),
            ("\"\"\n\"abc\"\n\"aaa\\\"aaa\"\n\"\\x27\"", 12),
        ];

        for (i, (input, result)) in tests.iter().enumerate() {
            assert_eq!(&puzzle1(input), result, "Test {} failed", i);
        }
    }

    #[test]
    fn puzzle2_test() {
        let tests = [
            ("\"\"", 4),
            ("\"abc\"", 4),
            ("\"aaa\\\"aaa\"", 6),
            ("\"\\x27\"", 5),
            ("\"\"\n\"abc\"\n\"aaa\\\"aaa\"\n\"\\x27\"", 19),
        ];

        for (i, (input, result)) in tests.iter().enumerate() {
            assert_eq!(&puzzle2(input), result, "Test {} failed", i);
        }
    }
}
