use core::panic;
use std::fs;

const NUM_STRS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn num_str_to_int(num: &str) -> isize {
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

fn first_digit(line: String) -> (isize, isize) {
    match line.chars().enumerate().find(|(_, c)| c.is_digit(10)) {
        Some((i, c)) => (i as isize, c.to_digit(10).unwrap() as isize),
        None => (-1, 0),
    }
}

fn last_digit(line: String) -> (isize, isize) {
    match line.chars().rev().enumerate().find(|(_, c)| c.is_digit(10)) {
        Some((i, c)) => (
            (line.len() - i - 1) as isize,
            c.to_digit(10).unwrap() as isize,
        ),
        None => (-1, 0),
    }
}

fn first_word(line: String) -> (isize, isize) {
    let mut best_match = (-1, "");
    for word in NUM_STRS {
        if !line.contains(word) {
            continue;
        }

        let index = line.find(word).unwrap() as isize;
        if best_match.1 == "" || index < best_match.0 {
            best_match = (index, word);
        }
    }

    if best_match.1 == "" {
        return (-1, 0);
    }

    let value = num_str_to_int(best_match.1);

    (best_match.0, value)
}

fn last_word(line: String) -> (isize, isize) {
    let mut best_match = (-1, "");
    for word in NUM_STRS {
        if !line.contains(word) {
            continue;
        }

        let index = line.rfind(word).unwrap() as isize;
        if best_match.1 == "" || index > best_match.0 {
            best_match = (index, word);
        }
    }

    if best_match.1 == "" {
        return (-1, 0);
    }

    let value = num_str_to_int(best_match.1);

    (best_match.0, value)
}

fn find_best(a: (isize, isize), b: (isize, isize), is_first: bool) -> isize {
    if a.0 == -1 && b.0 == -1 {
        panic!("Unable to find digit or word");
    }

    if is_first {
        if a.0 != -1 && a.0 < b.0 {
            a.1
        } else if b.0 == -1 {
            a.1
        } else {
            b.1
        }
    } else {
        if a.0 != -1 && a.0 > b.0 {
            a.1
        } else if b.0 == -1 {
            a.1
        } else {
            b.1
        }
    }
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    let mut sum = 0;
    for line in file.lines() {
        let first_digit = first_digit(String::from(line));
        let last_digit = last_digit(String::from(line));

        let first_word = first_word(String::from(line));
        let last_word = last_word(String::from(line));

        let first = find_best(first_digit, first_word, true);
        let last = find_best(last_digit, last_word, false);

        sum += format!("{}{}", first, last).parse::<i32>().unwrap();
    }

    println!("Sum: {sum}");
}
