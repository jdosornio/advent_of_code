use fancy_regex::Regex;

//Multiline input
pub fn puzzle1(input: &str) -> u32 {
    let nice_regex: Vec<Regex> = [
        r"(?:.*[aeiou].*){3}",
        //All are alpha but anyways
        r"([[:alpha:]])\1",
        r"(?:ab|cd|pq|xy)",
    ]
    .iter()
    .map(|exp| Regex::new(exp).unwrap())
    .collect();

    input.lines().fold(0, |total, line| {
        if nice_regex[0].is_match(line).unwrap()
            && nice_regex[1].is_match(line).unwrap()
            && !nice_regex[2].is_match(line).unwrap()
        {
            total + 1
        } else {
            total
        }
    })
}

pub fn puzzle2(input: &str) -> u32 {
    let nice_regex: Vec<Regex> = [
        //All are alpha but anyways
        r"([a-z]{2}).*\1",
        r"([a-z])[a-z]\1",
    ]
    .iter()
    .map(|exp| Regex::new(exp).unwrap())
    .collect();

    input.lines().fold(0, |total, line| {
        if nice_regex[0].is_match(line).unwrap() && nice_regex[1].is_match(line).unwrap() {
            total + 1
        } else {
            total
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn puzzle1_test() {
        let tests = [
            ("ugknbfddgicrmopn", 1),
            ("aaa", 1),
            ("jchzalrnumimnmhp", 0),
            ("haegwjzuvuyypxyu", 0),
            ("dvszwmarrgswjxmb", 0),
        ];

        for (i, (input, result)) in tests.iter().enumerate() {
            assert_eq!(&puzzle1(input), result, "Test {} failed", i);
        }
    }

    #[test]
    fn puzzle2_test() {
        let tests = [
            ("qjhvhtzxzqqjkmpb", 1),
            ("xxyxx", 1),
            ("uurcxstgmygtbstg", 0),
            ("ieodomkazucvgmuy", 0),
        ];

        for (i, (input, result)) in tests.iter().enumerate() {
            assert_eq!(&puzzle2(input), result, "Test {} failed", i);
        }
    }
}
