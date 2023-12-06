use std::{char, fs, usize};

#[derive(Debug, PartialEq)]
enum TokenType {
    Part(String),
    Value(usize),
}

#[derive(Debug)]
struct Token {
    token_type: TokenType,
    line_num: usize,
    index_range: (usize, usize),
}

fn tokenizer(chars: &Vec<char>, line_num: usize, index_range: (usize, usize)) -> Token {
    let val = String::from_iter(chars);
    let is_num = val.parse::<usize>();

    match is_num {
        Ok(val) => Token {
            token_type: TokenType::Value(val),
            line_num,
            index_range,
        },
        Err(_) => Token {
            token_type: TokenType::Part(String::from(val)),
            line_num,
            index_range,
        },
    }
}

fn evaluate_line(line: String, line_num: usize) -> Vec<Token> {
    let mut buf: Vec<char> = Vec::new();
    let mut start_index = 0;
    let mut chars = line.chars().enumerate();
    let mut tokens: Vec<Token> = Vec::new();

    while let Some((i, c)) = chars.next() {
        // Conditions for flushing
        let cond_a = c == '.';
        let cond_b = c.is_digit(10) && (&buf).len() > 0 && !(&buf).first().unwrap().is_digit(10);
        let cond_c = !c.is_digit(10) && (&buf).len() > 0 && (&buf).first().unwrap().is_digit(10);

        if cond_a || cond_b || cond_c {
            // Flush the buf into a token, reset for next item
            if (&buf).len() > 0 {
                let token = tokenizer(&buf, line_num, (start_index, i - 1));
                tokens.push(token);
            }

            // Reset
            buf.clear();

            if c == '.' {
                start_index = i + 1;
                continue;
            } else {
                start_index = i;
            }
        }

        buf.push(c)
    }

    if (&buf).len() > 0 {
        let token = tokenizer(&buf, line_num, (start_index, line.len() - 1));
        println!("{:?}", token);
        tokens.push(token);
    }

    println!("{:?}", tokens);
    return tokens;
}

fn tokens_in_proximity(part: &Token, other: &Token) -> bool {
    if let TokenType::Part(_) = other.token_type {
        return false;
    }

    let in_row_range = other.line_num >= part.line_num - 1 && other.line_num <= part.line_num + 1;
    let bot_col_range = if part.index_range.0 == 0 {
        0
    } else {
        part.index_range.0 - 1
    };
    let in_col_range =
        other.index_range.1 >= bot_col_range && other.index_range.0 <= part.index_range.0 + 1;

    in_row_range && in_col_range
}

fn evalute_token(token: &Token, tokens: &Vec<Token>) -> usize {
    // Ignore non-part tokens
    match &token.token_type {
        TokenType::Part(p) => {
            if p.chars().next().unwrap() != '*' {
                return 0;
            }
        }
        TokenType::Value(_) => return 0,
    }

    let tokens_iter = tokens.into_iter();
    let adjacents = tokens_iter
        .filter(|t| tokens_in_proximity(token, t))
        .map(|t| match t.token_type {
            TokenType::Part(_) => 0,
            TokenType::Value(i) => i,
        })
        .collect::<Vec<usize>>();

    if (&adjacents).len() == 2 {
        return adjacents.into_iter().product();
    }
    0
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    let mut tokens: Vec<Token> = Vec::new();
    for (i, line) in file.lines().enumerate() {
        tokens.extend(evaluate_line(String::from(line), i));
    }

    let mut sum = 0;
    for token in &tokens {
        sum += evalute_token(token, &tokens);
    }
    println!("Sum: {sum}");
}
