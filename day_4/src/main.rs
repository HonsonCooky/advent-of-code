use std::{collections::HashMap, fs, u32, usize};

#[derive(Debug)]
struct Card {
    title: usize,
    win_nums: Vec<usize>,
    our_nums: Vec<usize>,
}

impl Card {
    fn new(line: String) -> Card {
        let title = Card::title(&line);
        let win_nums = Card::winning_numbers(&line);
        let our_nums = Card::our_numbers(&line);
        Card {
            title,
            win_nums,
            our_nums,
        }
    }

    fn title(line: &String) -> usize {
        let mut split = line.split(":");
        let title_split = split.next().unwrap().split(" ").collect::<Vec<&str>>();
        title_split.last().unwrap().parse::<usize>().unwrap()
    }

    fn winning_numbers(line: &String) -> Vec<usize> {
        let first_range_index = line.find(":").unwrap() + 1;
        let last_range_index = line.find("|").unwrap();
        let vals = &line[first_range_index..last_range_index];
        vals.trim()
            .split(" ")
            .filter(|val| val.len() > 0 && val.chars().all(|c| c.is_digit(10)))
            .map(|val| val.parse::<usize>().unwrap())
            .collect::<Vec<usize>>()
    }

    fn our_numbers(line: &String) -> Vec<usize> {
        let first_range_index = line.find("|").unwrap() + 1;
        let vals = &line[first_range_index..];
        vals.trim()
            .split(" ")
            .filter(|val| val.len() > 0 && val.chars().all(|c| c.is_digit(10)))
            .map(|val| val.parse::<usize>().unwrap())
            .collect::<Vec<usize>>()
    }

    fn num_of_wins(&self) -> usize {
        let wins = &self.win_nums;
        let ours = &self.our_nums;

        wins.into_iter().filter(|wn| ours.contains(wn)).count()
    }
}

fn rec_find_win_cards(current: &Card, cards: &Vec<Card>, prev_registry: &mut Vec<Card>) {
    let wins = current.num_of_wins();
    if wins == 0 {
        return;
    }

    for i in (current.title + 1..=current.title + wins) {}

    return;
}

fn main() {
    let file = fs::read_to_string("sample.txt").unwrap();
    let mut cards: Vec<Card> = Vec::new();
    for line in file.lines() {
        cards.push(Card::new(String::from(line)));
    }

    let mut won_cards: HashMap<usize, usize> = HashMap::new();

    // Evaluate all "original" cards
    for card in &cards {
        rec_find_win_cards(card, &cards, &mut Vec::new());
    }

    // PART ONE
    // let mut sum = 0;
    // for card in &cards {
    //     let num_of_wins = card.num_of_wins();
    //     let pow = if num_of_wins > 0 {
    //         num_of_wins - 1
    //     } else {
    //         continue;
    //     };
    //     let points = (2 as usize).pow(pow as u32);
    //     println!("{:?} => {} = {}", card, card.num_of_wins(), points);
    //     sum += points;
    // }
    // println!("Sum: {sum}");
}
