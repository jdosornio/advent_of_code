use advent_of_code::{get_text, y2015};

fn main() {
    // TODO: Maybe add some macros
    //2015
    if let Ok(text) = get_text("inputs/2015/day1.txt") {
        println!(
            "2015, day 1, puzzle 1 answer: {}",
            y2015::day1::puzzle1(&text)
        );
        println!(
            "2015, day 1, puzzle 2 answer: {}",
            y2015::day1::puzzle2(&text)
        );
    }

    if let Ok(text) = get_text("inputs/2015/day2.txt") {
        println!(
            "2015, day 2, puzzle 1 answer: {}",
            y2015::day2::puzzle1(&text)
        );
        println!(
            "2015, day 2, puzzle 2 answer: {}",
            y2015::day2::puzzle2(&text)
        );
    }

    if let Ok(text) = get_text("inputs/2015/day3.txt") {
        println!(
            "2015, day 3, puzzle 1 answer: {}",
            y2015::day3::puzzle1(&text)
        );
        println!(
            "2015, day 3, puzzle 2 answer: {}",
            y2015::day3::puzzle2(&text)
        );
    }

    //Long run time
    // println!(
    //     "2015, day 4, puzzle 1 answer: {}",
    //     y2015::day4::puzzle("ckczppom", "00000")
    // );
    // println!(
    //     "2015, day 4, puzzle 2 answer: {}",
    //     y2015::day4::puzzle("ckczppom", "000000")
    // );

    if let Ok(text) = get_text("inputs/2015/day5.txt") {
        println!(
            "2015, day 5, puzzle 1 answer: {}",
            y2015::day5::puzzle1(&text)
        );
        println!(
            "2015, day 5, puzzle 2 answer: {}",
            y2015::day5::puzzle2(&text)
        );
    }

    if let Ok(text) = get_text("inputs/2015/day6.txt") {
        println!(
            "2015, day 6, puzzle 1 answer: {}",
            y2015::day6::puzzle1(&text)
        );
        println!(
            "2015, day 6, puzzle 2 answer: {}",
            y2015::day6::puzzle2(&text)
        );
    }

    // Long runtime
    // if let Ok(text) = get_text("inputs/2015/day7.txt") {
    //     println!(
    //         "2015, day 7, puzzle 1 answer: {}",
    //         y2015::day7::puzzle1(&text)
    //     );
    //     println!(
    //         "2015, day 7, puzzle 2 answer: {}",
    //         y2015::day7::puzzle2(&text)
    //     );
    // }

    if let Ok(text) = get_text("inputs/2015/day8.txt") {
        println!(
            "2015, day 8, puzzle 1 answer: {}",
            y2015::day8::puzzle1(&text)
        );
        println!(
            "2015, day 8, puzzle 2 answer: {}",
            y2015::day8::puzzle2(&text)
        );
    }

    if let Ok(text) = get_text("inputs/2015/day9.txt") {
        println!(
            "2015, day 9, puzzle 1 answer: {}",
            y2015::day9::puzzle1(&text)
        );
        // println!(
        //     "2015, day 9, puzzle 2 answer: {}",
        //     y2015::day9::puzzle2(&text)
        // );
    }
}
