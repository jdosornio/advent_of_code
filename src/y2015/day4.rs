use md5::{Digest, Md5};

fn hash(input: &str) -> String {
    let mut hasher = Md5::new();

    hasher.update(input);
    let result = hasher.finalize();

    format!("{:x}", result)
}

pub fn puzzle(input: &str, prefix: &str) -> u32 {
    let mut i = 1;
    loop {
        let input = format!("{}{}", input, i);
        let hash = hash(&input);

        if hash.starts_with(prefix) {
            break;
        }
        i += 1;
    }

    i
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn puzzle1_test() {
        let tests = [("abcdef", 609_043), ("pqrstuv", 1_048_970)];

        for (i, (input, result)) in tests.iter().enumerate() {
            assert_eq!(&puzzle(input, "00000"), result, "Test {} failed", i);
        }
    }
}
