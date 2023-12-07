use std::fs;

// For this extended challenge, the idea was to try do a "ONE-LINER" to solve the problem
// The only rule, is that you cannot use a semicolon (challenge came from a workmate)

fn main() {
    println!(
        "{:?}",
        fs::read_to_string("input.txt")
            .unwrap()
            .lines()
            .map(|line| line
                .chars()
                .enumerate()
                .map(|(i, c)| match c {
                    'o' =>
                        if line[i..].starts_with("one") {
                            '1'
                        } else {
                            'o'
                        },
                    't' =>
                        if line[i..].starts_with("two") {
                            '2'
                        } else if line[i..].starts_with("three") {
                            '3'
                        } else {
                            't'
                        },
                    'f' =>
                        if line[i..].starts_with("four") {
                            '4'
                        } else if line[i..].starts_with("five") {
                            '5'
                        } else {
                            'f'
                        },
                    's' =>
                        if line[i..].starts_with("six") {
                            '6'
                        } else if line[i..].starts_with("seven") {
                            '7'
                        } else {
                            's'
                        },
                    'e' =>
                        if line[i..].starts_with("eight") {
                            '8'
                        } else {
                            'e'
                        },
                    'n' =>
                        if line[i..].starts_with("nine") {
                            '9'
                        } else {
                            'n'
                        },
                    a => a,
                })
                .collect::<String>())
            .map(|line| format!(
                "{}{}",
                line.chars().find(|c| c.is_digit(10)).unwrap(),
                line.chars().rfind(|c| c.is_digit(10)).unwrap()
            )
            .parse::<usize>()
            .unwrap())
            .sum::<usize>() // .collect::<Vec<usize>>()
    );
}
