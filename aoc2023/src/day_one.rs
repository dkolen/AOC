use regex::Regex;
use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn day_one_part_one() -> i32 {
    let mut sum = 0;
    let mut d = String::from(env!("CARGO_MANIFEST_DIR"));
    d.push_str("/inputs/input1.txt");
    let file = match File::open(d) {
        Ok(file) => file,
        Err(why) => panic!("{}", why),
    };
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let mut left = 0;
        let mut right = 0;
        let mut lfound = false;
        let mut rfound = false;
        let re = Regex::new(r"^(\d)").unwrap();
        let text = match line {
            Ok(text) => text,
            Err(why) => panic!("{}", why),
        };
        let mut i = 0;
        let length = text.len();
        while i < length && !(lfound && rfound) {
            if !lfound && re.is_match(&text[i..i + 1]) {
                left = re
                    .captures(&text[i..i + 1])
                    .unwrap()
                    .get(1)
                    .unwrap()
                    .as_str()
                    .parse::<i32>()
                    .unwrap();
                lfound = true;
            }
            if !rfound && re.is_match(&text[length - i - 1..length - i]) {
                right = re
                    .captures(&text[length - i - 1..length - i])
                    .unwrap()
                    .get(1)
                    .unwrap()
                    .as_str()
                    .parse::<i32>()
                    .unwrap();
                rfound = true;
            }
            i += 1;
        }
        sum += 10 * left + right;
    }
    return sum;
}

pub fn day_one_part_two() -> i32 {
    let mut sum = 0;
    let mut d = String::from(env!("CARGO_MANIFEST_DIR"));
    d.push_str("/inputs/input1.txt");
    let file = match File::open(d) {
        Ok(file) => file,
        Err(why) => panic!("{}", why),
    };
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let mut left = 0;
        let mut right = 0;
        let mut lfound = false;
        let mut rfound = false;
        let re = Regex::new(r"^(\d|one|two|three|four|five|six|seven|eight|nine|zero)").unwrap();
        let text = match line {
            Ok(text) => text,
            Err(why) => panic!("{}", why),
        };
        let mut i = 0;
        let length = text.len();
        while i < length && !(lfound && rfound) {
            if !lfound && re.is_match(&text[i..]) {
                let left_str = re.captures(&text[i..]).unwrap().get(1).unwrap().as_str();
                if left_str.len() == 1 {
                    left = left_str.parse::<i32>().unwrap();
                } else {
                    left = get_num_from_str(left_str);
                }
                lfound = true;
            }
            if !rfound && re.is_match(&text[length - i - 1..]) {
                let right_str = re
                    .captures(&text[length - i - 1..])
                    .unwrap()
                    .get(1)
                    .unwrap()
                    .as_str();
                if right_str.len() == 1 {
                    right = right_str.parse::<i32>().unwrap();
                } else {
                    right = get_num_from_str(right_str);
                }
                rfound = true;
            }
            i += 1;
        }
        sum += 10 * left + right;
    }
    return sum;
}

fn get_num_from_str(str: &str) -> i32 {
    if str == "one" {
        1
    } else if str == "two" {
        2
    } else if str == "three" {
        3
    } else if str == "four" {
        4
    } else if str == "five" {
        5
    } else if str == "six" {
        6
    } else if str == "seven" {
        7
    } else if str == "eight" {
        8
    } else if str == "nine" {
        9
    } else if str == "zero" {
        0
    } else {
        panic!();
    }
}
