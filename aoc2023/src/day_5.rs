use regex::Regex;
use std::cmp::max;
use std::cmp::min;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::u64::MAX;

pub fn d5p1() -> u64 {
    let mut d = String::from(env!("CARGO_MANIFEST_DIR"));
    d.push_str("/inputs/input5.txt");
    let file = match File::open(d) {
        Ok(file) => file,
        Err(why) => panic!("{}", why),
    };
    let reader = BufReader::new(file);

    let mut seeds_gathered = false;
    let mut seeds: Vec<(u64, bool)> = Vec::new();
    let re = Regex::new(r"^(\d+)( \d+){2}$").unwrap();

    for line in reader.lines() {
        let text = match line {
            Ok(text) => text,
            Err(why) => panic!("{}", why),
        };
        if !seeds_gathered {
            let seeds_strs: Vec<&str> = text.split(" ").collect();
            for idx in 1..seeds_strs.len() {
                seeds.push((seeds_strs[idx].parse::<u64>().unwrap(), false));
            }
            seeds_gathered = true;
        } else if re.is_match(&text) {
            let map_strs: Vec<&str> = text.split(" ").collect();
            let dest = map_strs[0].parse::<u64>().unwrap();
            let src = map_strs[1].parse::<u64>().unwrap();
            let range = map_strs[2].parse::<u64>().unwrap();
            for idx in 0..seeds.len() {
                match &seeds[idx] {
                    (val, false) => {
                        if val >= &src && val < &(src + range) {
                            seeds[idx] = (dest + (val - src), true);
                        }
                    }
                    _ => {}
                };
            }
        } else if text.len() == 0 {
            for idx in 0..seeds.len() {
                let (val, _) = seeds[idx];
                seeds[idx] = (val, false);
            }
        }
    }
    let mut min = MAX;
    for seed in seeds {
        let (val, _) = seed;
        if val < min {
            min = val;
        }
    }
    min
}

pub fn d5p2() -> u64 {
    let mut d = String::from(env!("CARGO_MANIFEST_DIR"));
    d.push_str("/inputs/input5.txt");
    let file = match File::open(d) {
        Ok(file) => file,
        Err(why) => panic!("{}", why),
    };
    let reader = BufReader::new(file);

    let mut seeds_gathered = false;

    let mut seeds: Vec<(u64, u64, bool)> = Vec::new();
    let re = Regex::new(r"^(\d+)( \d+){2}$").unwrap();

    for line in reader.lines() {
        let text = match line {
            Ok(text) => text,
            Err(why) => panic!("{}", why),
        };
        if !seeds_gathered {
            let seeds_strs: Vec<&str> = text.split(" ").collect();
            for idx in 1..seeds_strs.len() - 1 {
                if idx % 2 == 1 {
                    let start = seeds_strs[idx].parse::<u64>().unwrap();
                    let range = seeds_strs[idx + 1].parse::<u64>().unwrap();
                    seeds.push((start, start + range, false));
                }
            }
            seeds_gathered = true;
        } else if re.is_match(&text) {
            let mut new_seeds: Vec<(u64, u64, bool)> = Vec::new();
            while seeds.len() > 0 {
                let (start, end, mapped) = seeds.pop().unwrap();
                if mapped {
                    new_seeds.push((start, end, mapped));
                } else {
                    let map_strs: Vec<&str> = text.split(" ").collect();
                    let dest = map_strs[0].parse::<u64>().unwrap();
                    let src = map_strs[1].parse::<u64>().unwrap();
                    let range = map_strs[2].parse::<u64>().unwrap();
                    if start >= src && end <= src + range {
                        new_seeds.push((start - src + dest, end - src + dest, true));
                    } else if start >= src + range || end <= src {
                        new_seeds.push((start, end, false));
                    } else {
                        seeds.push((max(src, start), min(src + range, end), false));
                        if end > src + range {
                            seeds.push((src + range, end, false));
                        }
                        if start < src {
                            seeds.push((start, src, false));
                        }
                    }
                }
            }
            seeds = new_seeds.clone();
        } else if text.len() == 0 {
            for i in 0..seeds.len() {
                let (start, end, _) = seeds[i];
                seeds[i] = (start, end, false);
            }
        }
    }
    let mut result: u64 = MAX;
    for seed in seeds {
        let (start, _, __) = seed;
        result = min(start, result);
    }
    return result;
}

/*
This works but is very inefficient, need a new soln

pub fn d5p2() -> u64 {
    let mut d = String::from(env!("CARGO_MANIFEST_DIR"));
    d.push_str("/inputs/input5.txt");
    let file = match File::open(d) {
        Ok(file) => file,
        Err(why) => panic!("{}", why),
    };
    let reader = BufReader::new(file);

    let mut seeds_gathered = false;

    let mut seeds: Vec<(u64, bool)> = Vec::new();
    let re = Regex::new(r"^(\d+)( \d+){2}$").unwrap();
    let mut line_num = 1;

    for line in reader.lines() {
        println!("Line: {}", line_num);
        line_num += 1;
        let text = match line {
            Ok(text) => text,
            Err(why) => panic!("{}", why),
        };
        if !seeds_gathered {
            let seeds_strs: Vec<&str> = text.split(" ").collect();
            for idx in 1..seeds_strs.len() - 1 {
                if idx % 2 == 1 {
                    let start = seeds_strs[idx].parse::<u64>().unwrap();
                    let range = seeds_strs[idx + 1].parse::<u64>().unwrap();
                    for j in 0..range {
                        seeds.push((start + j, false));
                    }
                }
            }
            seeds_gathered = true;
        } else if re.is_match(&text) {
            let map_strs: Vec<&str> = text.split(" ").collect();
            let dest = map_strs[0].parse::<u64>().unwrap();
            let src = map_strs[1].parse::<u64>().unwrap();
            let range = map_strs[2].parse::<u64>().unwrap();
            for idx in 0..seeds.len() {
                match &seeds[idx] {
                    (val, false) => {
                        if val >= &src && val < &(src + range) {
                            seeds[idx] = (dest + (val - src), true);
                        }
                    }
                    _ => {}
                };
            }
        } else if text.len() == 0 {
            for idx in 0..seeds.len() {
                let (val, _) = seeds[idx];
                seeds[idx] = (val, false);
            }
        }
    }
    let mut min = MAX;
    for seed in seeds {
        let (val, _) = seed;
        if val < min {
            min = val;
        }
    }
    min
}
*/
