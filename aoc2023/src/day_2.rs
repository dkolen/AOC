use regex::Regex;
use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn day_two_part_one() -> i32 {
    let mut d = String::from(env!("CARGO_MANIFEST_DIR"));
    d.push_str("/inputs/input2.txt");
    let file = match File::open(d) {
        Ok(file) => file,
        Err(why) => panic!("{}", why),
    };
    let reader = BufReader::new(file);
    let mut sum = 0;
    let re = Regex::new(r"^Game (\d+): (((1[0-2]|[0-9]) red|(1[0-3]|[0-9]) green|(1[0-4]|[0-9]) blue)(, ((1[0-2]|[0-9]) red|(1[0-3]|[0-9]) green|(1[0-4]|[0-9]) blue))*)(; (((1[0-2]|[0-9]) red|(1[0-3]|[0-9]) green|(1[0-4]|[0-9]) blue)(, ((1[0-2]|[0-9]) red|(1[0-3]|[0-9]) green|(1[0-4]|[0-9]) blue))*))*$").unwrap();
    for line in reader.lines() {
        let text = match line {
            Ok(text) => text,
            Err(why) => panic!("{}", why),
        };
        if re.is_match(&text) {
            let id = re
                .captures(&text)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .parse::<i32>()
                .unwrap();
            sum += id;
        }
    }
    sum
}

pub fn day_two_part_two() -> i32 {
    let mut d = String::from(env!("CARGO_MANIFEST_DIR"));
    d.push_str("/inputs/input2.txt");
    let file = match File::open(d) {
        Ok(file) => file,
        Err(why) => panic!("{}", why),
    };
    let reader = BufReader::new(file);
    let mut sum = 0;
    let re = Regex::new(r"(Game \d+:)").unwrap();
    let re2 = Regex::new(r"( (\d+) (red|green|blue)[,;]?)").unwrap();
    for line in reader.lines() {
        let text = match line {
            Ok(text) => text,
            Err(why) => panic!("{}", why),
        };
        let mut pos = 0;
        let caps1 = re.captures(&text).unwrap();
        pos += caps1.get(1).unwrap().as_str().len();
        let mut counts = [0, 0, 0];
        while pos < text.len() {
            let count: i32;
            let caps2 = re2.captures(&text[pos..]).unwrap();
            pos += caps2.get(1).unwrap().as_str().len();
            if caps2.get(3).unwrap().as_str() == "red" {
                count = caps2.get(2).unwrap().as_str().parse::<i32>().unwrap();
                if counts[0] < count {
                    counts[0] = count;
                }
            } else if caps2.get(3).unwrap().as_str() == "green" {
                count = caps2.get(2).unwrap().as_str().parse::<i32>().unwrap();
                if counts[1] < count {
                    counts[1] = count;
                }
            } else {
                count = caps2.get(2).unwrap().as_str().parse::<i32>().unwrap();
                if counts[2] < count {
                    counts[2] = count;
                }
            }
        }
        sum += counts[0] * counts[1] * counts[2];
    }
    sum
}
