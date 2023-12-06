use core::panic;
use std::{env, fs};

#[derive(Debug)]
struct DrawConfig {
    red: usize,
    green: usize,
    blue: usize,
}

#[derive(Debug)]
struct Game {
    id: usize,
    draws: Vec<DrawConfig>,
}

impl Game {
    fn new(line: &str) -> Game {
        Game {
            id: get_game_id(String::from(line)),
            draws: get_game_sets(String::from(line)),
        }
    }

    fn winner(&self, game_config: &DrawConfig) -> bool {
        for draw in &self.draws {
            if draw.red > game_config.red
                || draw.green > game_config.green
                || draw.blue > game_config.blue
            {
                return false;
            }
        }
        return true;
    }

    fn min_config(&self) -> usize {
        let mut red = 0;
        let mut blue = 0;
        let mut green = 0;
        for draw in &self.draws {
            if draw.red > red {
                red = draw.red;
            }
            if draw.green > green {
                green = draw.green;
            }
            if draw.blue > blue {
                blue = draw.blue
            }
        }
        red * green * blue
    }
}

fn get_game_id(input: String) -> usize {
    let lower_id_index = input.find(":").unwrap();
    let upper_id_index = input
        .chars()
        .enumerate()
        .find(|(_, c)| c.is_digit(10))
        .unwrap()
        .0;

    input[upper_id_index..lower_id_index]
        .parse::<usize>()
        .unwrap()
}

fn build_draw_config(input: &str) -> DrawConfig {
    let cubes = input
        .split(",")
        .map(|c| String::from(c.trim()))
        .collect::<Vec<String>>();

    let mut config = DrawConfig {
        red: 0,
        green: 0,
        blue: 0,
    };

    for cube in cubes {
        let vals = cube.split(" ").collect::<Vec<&str>>();
        if vals.len() > 2 {
            panic!("Too many values")
        }
        let num = vals[0].parse::<usize>().unwrap();
        let color = vals[1];

        match color {
            "red" => config.red = num,
            "green" => config.green = num,
            "blue" => config.blue = num,
            _ => panic!("Found weird color: '{color}'"),
        };
    }

    config
}

fn get_game_sets(input: String) -> Vec<DrawConfig> {
    let lower_id_index = input.find(":").unwrap();
    input[lower_id_index + 1..]
        .trim()
        .split(';')
        .map(build_draw_config)
        .collect::<Vec<DrawConfig>>()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = DrawConfig {
        red: args.get(1).unwrap().parse::<usize>().unwrap(),
        green: args.get(2).unwrap().parse::<usize>().unwrap(),
        blue: args.get(3).unwrap().parse::<usize>().unwrap(),
    };

    let file = fs::read_to_string("input.txt").unwrap();
    let mut sum = 0;

    for line in file.lines() {
        println!("LINE: {}", line);
        let game = Game::new(line);
        sum += game.min_config();
    }

    println!("Sum: {sum}");
}
