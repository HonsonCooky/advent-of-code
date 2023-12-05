use std::{fmt::format, fs, usize};

struct Digit {
    value: usize,
    index: isize,
    is_valid: bool,
}

struct Answer {
    first: Digit,
    second: Digit,
    value: usize,
}

// --------------------------------------------------------------------------------------------------------------------
// PART ONE
// --------------------------------------------------------------------------------------------------------------------

fn find_first_digit(line: &String) -> Digit {
    let index: isize = match line.find(|c: char| c.is_digit(10)) {
        Some(i) => i as isize,
        _ => -1,
    };

    let value = if index == -1 {
        0
    } else {
        line.chars()
            .nth(index as usize)
            .unwrap()
            .to_digit(10)
            .unwrap() as usize
    };

    Digit {
        value,
        index,
        is_valid: index != -1,
    }
}

fn part_one(line: &String) -> Answer {
    let first = find_first_digit(line);
    let second = find_first_digit(&line.chars().rev().collect::<String>());
    let value = format!("{}{}", first.value, second.value)
        .parse::<usize>()
        .unwrap_or(0);
    Answer {
        first,
        second,
        value,
    }
}

// --------------------------------------------------------------------------------------------------------------------
// PART TWO
// --------------------------------------------------------------------------------------------------------------------

const NUM_STRS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn num_str_to_int(num: &str) -> usize {
    match num {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => panic!("Num: {num}, is not a valid num"),
    }
}

fn find_num_str(line: &String, rev: bool) -> Digit {
    let mut best_match = (0, "");
    for num in NUM_STRS {
        if !line.contains(num) {
            continue;
        }

        if rev {
            let index = line.rfind(num).unwrap();
            if best_match.1 == "" || best_match.0 < index {
                best_match = (index, num);
            }
        } else {
            let index = line.find(num).unwrap();
            if best_match.1 == "" || best_match.0 > index {
                best_match = (index, num);
            }
        }
    }

    if best_match.1 == "" {
        Digit {
            value: 0,
            index: -1,
            is_valid: false,
        }
    } else {
        Digit {
            value: num_str_to_int(best_match.1),
            index: best_match.0 as isize,
            is_valid: true,
        }
    }
}

fn part_two(line: &String) -> Answer {
    let lower_case = line.to_lowercase();
    let first = find_num_str(&lower_case, false);
    let second = find_num_str(&lower_case, true);
    let value = format!("{}{}", first.value, second.value)
        .parse::<usize>()
        .unwrap_or(0);

    Answer {
        first,
        second,
        value,
    }
}

// --------------------------------------------------------------------------------------------------------------------
// MAIN
// --------------------------------------------------------------------------------------------------------------------

fn find_val(digits: Answer, words: Answer) {}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    let mut sum = 0;
    for line in file.lines() {
        let part_one = part_one(&String::from(line));
        let part_two = part_two(&String::from(line));
    }

    println!("Sum: {sum}");
}
