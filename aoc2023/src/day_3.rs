use regex::Regex;
use std::cmp::min;
use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn day_three_part_one() -> i32 {
    let mut d = String::from(env!("CARGO_MANIFEST_DIR"));
    d.push_str("/inputs/input3.txt");
    let file = match File::open(d) {
        Ok(file) => file,
        Err(why) => panic!("{}", why),
    };
    let reader = BufReader::new(file);
    let mut sum = 0;
    let re = Regex::new(r"^(\d+)").unwrap();
    let mut str_total = String::from("");
    for line in reader.lines() {
        let text = match line {
            Ok(text) => text,
            Err(why) => panic!("{}", why),
        };
        str_total.push_str(&text);
        str_total.push_str(" ");
    }
    let rows: Vec<&str> = str_total.split(" ").collect();
    for i in 0..rows.len() - 1 {
        let mut pos = 0;
        let curr = rows.get(i).unwrap();
        while pos < curr.len() {
            let m = re.captures(&curr[pos..]);
            if !m.is_none() {
                let id = m.unwrap().get(1).unwrap().as_str().parse::<i32>().unwrap();
                let length = id.to_string().len();
                let check_above = i >= 1;
                let check_below = i + 1 < rows.len() - 1;
                let mut left = 0;
                if pos > 1 {
                    left = pos - 1;
                }
                let right = min(curr.len() - 1, pos + length + 1);
                let mut is_part = false;
                if check_above {
                    for j in left..right {
                        if &rows.get(i - 1).unwrap()[j..j + 1] != "." {
                            is_part = true;
                        }
                    }
                }
                if check_below && !is_part {
                    for j in left..right {
                        if &rows.get(i + 1).unwrap()[j..j + 1] != "." {
                            is_part = true;
                        }
                    }
                }
                if pos >= 1 && !is_part {
                    if &curr[pos - 1..pos] != "." {
                        is_part = true;
                    }
                }
                if pos + length < curr.len() && !is_part {
                    if &curr[pos + length..pos + length + 1] != "." {
                        is_part = true;
                    }
                }
                if is_part {
                    sum += id;
                }
                pos += length;
            } else {
                pos += 1;
            }
        }
    }
    sum
}

pub fn day_three_part_two() -> i32 {
    let mut d = String::from(env!("CARGO_MANIFEST_DIR"));
    d.push_str("/inputs/input3.txt");
    let file = match File::open(d) {
        Ok(file) => file,
        Err(why) => panic!("{}", why),
    };
    let reader = BufReader::new(file);
    let mut sum = 0;
    let mut str_total = String::from("");
    let re = Regex::new(r"[^0123456789.]").unwrap();
    let digit_re = Regex::new(r"^(\d+)").unwrap();
    for line in reader.lines() {
        let text = match line {
            Ok(text) => text,
            Err(why) => panic!("{}", why),
        };
        str_total.push_str(&text);
        str_total.push_str(" ");
    }
    let rows: Vec<&str> = str_total.split(" ").collect();
    for i in 0..rows.len() - 1 {
        let mut pos = 0;
        let curr = rows.get(i).unwrap();
        while pos < curr.len() {
            if re.is_match(&curr[pos..pos + 1]) {
                let mut adj_vec: Vec<i32> = Vec::new();
                //let left = max(0, pos - 1);
                let right = min(curr.len() - 1, pos + 1);
                if i > 0 {
                    let mut j = pos;
                    while j <= right + 1 {
                        while j > 0 && digit_re.is_match(&rows.get(i - 1).unwrap()[j - 1..]) {
                            j -= 1;
                        }
                        if j <= right {
                            let m = digit_re.captures(&rows.get(i - 1).unwrap()[j..]);
                            if m.is_some() {
                                let num =
                                    m.unwrap().get(1).unwrap().as_str().parse::<i32>().unwrap();
                                let num_len = num.to_string().len();
                                adj_vec.push(num);
                                j += num_len + 1;
                            } else {
                                j += 1;
                            }
                        } else {
                            j += 1;
                        }
                    }
                }
                if pos > 0 {
                    let mut j = pos;
                    while j > 0 && digit_re.is_match(&rows.get(i).unwrap()[j - 1..]) {
                        j -= 1;
                    }
                    let m = digit_re.captures(&rows.get(i).unwrap()[j..]);
                    if m.is_some() {
                        let num = m.unwrap().get(1).unwrap().as_str().parse::<i32>().unwrap();
                        adj_vec.push(num);
                    }
                }
                if pos < curr.len() - 1 {
                    let m = digit_re.captures(&rows.get(i).unwrap()[pos + 1..]);
                    if m.is_some() {
                        let num = m.unwrap().get(1).unwrap().as_str().parse::<i32>().unwrap();
                        adj_vec.push(num);
                    }
                }
                if i < rows.len() - 1 {
                    let mut j = pos;
                    while j <= right + 1 {
                        while j > 0 && digit_re.is_match(&rows.get(i + 1).unwrap()[j - 1..]) {
                            j -= 1;
                        }
                        if j <= right {
                            let m = digit_re.captures(&rows.get(i + 1).unwrap()[j..]);
                            if m.is_some() {
                                let num =
                                    m.unwrap().get(1).unwrap().as_str().parse::<i32>().unwrap();
                                let num_len = num.to_string().len();
                                adj_vec.push(num);
                                j += num_len + 1;
                            } else {
                                j += 1;
                            }
                        } else {
                            j += 1;
                        }
                    }
                }
                if adj_vec.len() == 2 {
                    sum += adj_vec.get(0).unwrap() * adj_vec.get(1).unwrap();
                }
            }
            pos += 1;
        }
    }
    sum
}
