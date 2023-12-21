use std::cmp::min;
use std::collections::HashMap;

fn main() {
    //let data = include_str!("../example1.txt");
    let data = include_str!("../input.txt");
    part_a(data);
    part_b(data);
}

#[allow(dead_code)]
fn part_a(data: &str) {
    let mut sum = 0;
    for line in data.lines() {
        let mut winning_numbers = Vec::new();
        let mut card_numbers = Vec::new();
        // split on ": " and just get the second part
        let parts: Vec<&str> = line.split(": ").nth(1).unwrap().split(" | ").collect();
        parts[0].split_whitespace().for_each(|n| {
            winning_numbers.push(n.parse::<u32>().unwrap());
        });
        parts[1].split_whitespace().for_each(|n| {
            card_numbers.push(n.parse::<u32>().unwrap());
        });
        let card_value = card_numbers.iter().fold(0, |acc, n| {
            // one winning number gets 1 point, each additional winning number doubles the points
            if winning_numbers.contains(n) {
                match acc {
                    0 => 1,
                    _ => acc * 2,
                }
            } else {
                acc
            }
        });
        sum += card_value;
    }
    println!("sum: {}", sum);
}

fn process_line_b(line: &str) -> u32 {
    // process a line for part b and return the count of winning numbers for this game
    let (winners, this_card) = get_card_parts(line);
    count_matches(&winners, &this_card)
}

fn get_card_parts(line: &str) -> (Vec<u32>, Vec<u32>) {
    // return a tuple of the winning numbers and the card numbers for this game
    let mut winning_numbers = Vec::new();
    let mut card_numbers = Vec::new();
    let (first, second) = line.split(": ").nth(1).unwrap().split_once(" | ").unwrap();
    first.split_whitespace().for_each(|n| {
        winning_numbers.push(n.parse::<u32>().unwrap());
    });
    second.split_whitespace().for_each(|n| {
        card_numbers.push(n.parse::<u32>().unwrap());
    });
    (winning_numbers, card_numbers)
}

fn count_matches(winning_numbers: &Vec<u32>, card_numbers: &Vec<u32>) -> u32 {
    // just count how many of the card numbers are in the winning numbers
    card_numbers.iter().fold(0, |acc, n| {
        if winning_numbers.contains(n) {
            acc + 1
        } else {
            acc
        }
    })
}

trait SmartIncrement {
    fn increment_or_set(&mut self, key: u32);
}
impl SmartIncrement for HashMap<u32, u32> {
    //add a method to increment a value or set it to 1 if it doesn't exist
    fn increment_or_set(&mut self, key: u32) {
        match self.get(&key) {
            Some(val) => self.insert(key, val + 1),
            None => self.insert(key, 1),
        };
    }
}

fn part_b(data: &str) {
    const NUM_CARDS: usize = 218;
    let lines: Vec<&str> = data.lines().collect();
    let mut card_arr: [u32; NUM_CARDS] = [1; NUM_CARDS];
    //card_map.insert(1, 1);
    let mut game_num = 0;
    for line in lines.iter() {
        let this_score = process_line_b(line);
        if this_score > 0 {
            println!("game {game_num} won {this_score} cards");
            let last_index = min(game_num + this_score, NUM_CARDS as u32 - 1);
            (game_num + 1..=last_index).for_each(|n| {
                card_arr[n as usize] += card_arr[game_num as usize];
            });
        }
        println!("{game_num}: card_arr: {:?}", card_arr);
        game_num += 1;
    }
    let score: u32 = card_arr.iter().sum();
    println!("score: {score}");
}
