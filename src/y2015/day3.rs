use std::collections::HashSet;
use std::iter::FromIterator;

#[derive(Eq, PartialEq, Hash, Clone)]
struct Point(i32, i32);

pub fn puzzle1(input: &str) -> usize {
    let mut current_house = Point(0, 0);
    let mut visited_houses = HashSet::<Point>::from_iter([current_house.clone()].to_vec());

    input.chars().for_each(|dir| {
        match dir {
            '<' => {
                current_house.0 -= 1;
            }
            '>' => {
                current_house.0 += 1;
            }
            '^' => {
                current_house.1 += 1;
            }
            'v' => {
                current_house.1 -= 1;
            }
            _ => {}
        }
        visited_houses.insert(current_house.clone());
    });
    visited_houses.len()
}

pub fn puzzle2(input: &str) -> usize {
    let mut current_house_santa = Point(0, 0);
    let mut current_house_robot = Point(0, 0);
    let mut visited_houses = HashSet::<Point>::from_iter([current_house_santa.clone()].to_vec());

    input.chars().enumerate().for_each(|(i, dir)| {
        let current_house = if i % 2 == 0 {
            &mut current_house_santa
        } else {
            &mut current_house_robot
        };
        match dir {
            '<' => {
                current_house.0 -= 1;
            }
            '>' => {
                current_house.0 += 1;
            }
            '^' => {
                current_house.1 += 1;
            }
            'v' => {
                current_house.1 -= 1;
            }
            _ => {}
        }
        visited_houses.insert(current_house_santa.clone());
        visited_houses.insert(current_house_robot.clone());
    });
    visited_houses.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn puzzle1_test() {
        let tests = [(">", 2), ("^>v<", 4), ("^v^v^v^v^v", 2)];

        for (i, (input, result)) in tests.iter().enumerate() {
            assert_eq!(&puzzle1(input), result, "Test {} failed", i);
        }
    }

    #[test]
    fn puzzle2_test() {
        let tests = [("^v", 3), ("^>v<", 3), ("^v^v^v^v^v", 11)];

        for (i, (input, result)) in tests.iter().enumerate() {
            assert_eq!(&puzzle2(input), result, "Test {} failed", i);
        }
    }
}
