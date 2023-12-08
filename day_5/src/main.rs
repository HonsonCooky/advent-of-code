use std::{collections::HashMap, fs, iter::Peekable, path::Iter, str::Lines, usize, fmt::Display};

fn get_seeds<'a, I>(lines: &mut I) -> Vec<usize> 
where I:Iterator<Item = &'a str>, {
    let line = lines.next().unwrap();
    if !line.starts_with("seeds") {
        panic!("Not a seeds line")
    }

    line.split(":")
        .last()
        .unwrap()
        .trim()
        .split(" ")
        .map(|n| n.parse().unwrap())
        .collect()
}

fn make_mapping(lines: Vec<String>) {}

fn main() {
    let file = fs::read_to_string("sample.txt").unwrap();
    let mut lines = file.lines();
    let seeds = get_seeds(&mut lines);

    while let Some(line) = lines.next() {
        let partition = lines.partition(|line| )
        println!("{:?}", collection);
    }
}
