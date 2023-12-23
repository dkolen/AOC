use regex::Regex;
use std::cmp::max;
use std::cmp::min;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::u64::MAX;

/*
    Thought Process:
    Distance traveled = speed * (time - speed)
    let x = speed
    f(x) = distance traveled
    Case #1: Time = 7 ms
    f(x) = x * (7 - x)
    f(x) = 7x - x^2
    Max distance is where f'(x) = 0
    f'(x) = 7 - 2x
    7 - 2x = 0 -> x = 3.5
    Start at speed that gives the max distance then increase and decrease speed
    until you no longer win the race.
*/

fn distance(time: u64, spd: u64) -> u64 {
    return time * spd - spd * spd;
}

fn max_speed(time: u64) -> u64 {
    let floor = time / 2;
    let ceiling = time / 2 + 1;
    if distance(time, floor) > distance(time, ceiling) {
        return floor;
    } else {
        return ceiling;
    }
}

pub fn d6p1() -> u64 {
    let mut d = String::from(env!("CARGO_MANIFEST_DIR"));
    d.push_str("/inputs/input6.txt");
    let file = match File::open(d) {
        Ok(file) => file,
        Err(why) => panic!("{}", why),
    };
    let reader = BufReader::new(file);

    let mut time: Vec<u64> = Vec::new();
    let mut dist: Vec<u64> = Vec::new();
    let mut ways_to_win: Vec<u64> = Vec::new();

    for line in reader.lines() {
        let text = match line {
            Ok(text) => text,
            Err(why) => panic!("{}", why),
        };
        let split: Vec<&str> = text.split(" ").collect();
        if split[0] == "Time:" {
            for i in 1..split.len() {
                if split[i].len() > 0 {
                    time.push(split[i].parse::<u64>().unwrap());
                }
            }
        } else {
            for i in 1..split.len() {
                if split[i].len() > 0 {
                    dist.push(split[i].parse::<u64>().unwrap());
                }
            }
        }
    }
    for i in 0..time.len() {
        let max_speed: u64 = max_speed(time[i]);
        let mut upper = max_speed;
        let mut lower = max_speed;
        while distance(time[i], upper + 1) > dist[i] {
            upper += 1;
        }
        while lower > 0 && distance(time[i], lower - 1) > dist[i] {
            lower -= 1;
        }
        if distance(time[i], upper) > dist[i] {
            ways_to_win.push(upper - lower + 1);
        } else {
            ways_to_win.push(0)
        }
    }
    let mut result = ways_to_win[0];
    for i in 1..ways_to_win.len() {
        result *= ways_to_win[i];
    }
    return result;
}

pub fn d6p2() -> u64 {
    let mut d = String::from(env!("CARGO_MANIFEST_DIR"));
    d.push_str("/inputs/input6.txt");
    let file = match File::open(d) {
        Ok(file) => file,
        Err(why) => panic!("{}", why),
    };
    let reader = BufReader::new(file);

    let mut time = String::from("");
    let mut dist = String::from("");

    for line in reader.lines() {
        let text = match line {
            Ok(text) => text,
            Err(why) => panic!("{}", why),
        };
        let split: Vec<&str> = text.split(" ").collect();
        if split[0] == "Time:" {
            for i in 1..split.len() {
                if split[i].len() > 0 {
                    time.push_str(split[i]);
                }
            }
        } else {
            for i in 1..split.len() {
                if split[i].len() > 0 {
                    dist.push_str(split[i]);
                }
            }
        }
    }
    let time: u64 = time.parse::<u64>().unwrap();
    let dist: u64 = dist.parse::<u64>().unwrap();
    let max_speed: u64 = max_speed(time).into();
    let mut upper = max_speed;
    let mut lower = max_speed;
    while distance(time, upper + 1) > dist {
        upper += 1;
    }
    while lower > 0 && distance(time, lower - 1) > dist {
        lower -= 1;
    }
    if distance(time, upper) > dist {
        return upper - lower + 1;
    } else {
        return 0;
    }
}
