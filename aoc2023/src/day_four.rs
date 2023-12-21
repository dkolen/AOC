use regex::Regex;
use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn day_four_part_one() -> i32 {
    let mut sum = 0;
    let mut d = String::from(env!("CARGO_MANIFEST_DIR"));
    d.push_str("/inputs/input4.txt");
    let file = match File::open(d) {
        Ok(file) => file,
        Err(why) => panic!("{}", why),
    };
    let reader = BufReader::new(file);
    let re = Regex::new(r"(Card +\d+: +)").unwrap();
    for line in reader.lines() {
        let mut winners: u32 = 0;
        let text = match line {
            Ok(text) => text,
            Err(why) => panic!("{}", why),
        };
        let length = re.captures(&text).unwrap().get(1).unwrap().as_str().len();
        let input: Vec<&str> = text[length..].split(" ").collect();
        let index = input.iter().position(|&r| r == "|").unwrap();
        for i in 0..index {
            for j in index + 1..input.len() {
                if input[i] != "" && input[i] == input[j] {
                    winners += 1;
                }
            }
        }
        if winners > 0 {
            let base: i32 = 2;
            sum += base.pow(winners - 1);
        }
    }
    sum
}

pub fn day_four_part_two() -> i32 {
    let mut d = String::from(env!("CARGO_MANIFEST_DIR"));
    d.push_str("/inputs/input4.txt");
    let file = match File::open(&d) {
        Ok(file) => file,
        Err(why) => panic!("{}", why),
    };
    let reader = BufReader::new(file);
    let line_count = reader.lines().count();
    let file = match File::open(&d) {
        Ok(file) => file,
        Err(why) => panic!("{}", why),
    };
    let reader = BufReader::new(file);
    let re = Regex::new(r"(Card +\d+: +)").unwrap();
    let mut card_counts = vec![1; line_count];
    let mut idx: u32 = 0;
    for line in reader.lines() {
        let text = match line {
            Ok(text) => text,
            Err(why) => panic!("{}", why),
        };
        let mut winners: u32 = 0;
        let length = re.captures(&text).unwrap().get(1).unwrap().as_str().len();
        let input: Vec<&str> = text[length..].split(" ").collect();
        let index = input.iter().position(|&r| r == "|").unwrap();
        for i in 0..index {
            for j in index + 1..input.len() {
                if input[i] != "" && input[i] == input[j] {
                    winners += 1;
                }
            }
        }
        for j in idx + 1..idx + 1 + winners {
            card_counts[j as usize] = card_counts[j as usize] + card_counts[idx as usize];
        }
        idx += 1;
    }
    let mut sum = 0;
    for num in card_counts {
        sum += num;
    }
    sum
}
