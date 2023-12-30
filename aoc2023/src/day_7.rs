use std::cmp::Ord;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Clone)]
struct Hand {
    cards_str: String,
    cards_vec: Vec<(String, u32)>,
    bid: u32,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if &self.cards_vec.len() > &other.cards_vec.len() {
            return Ordering::Less;
        } else if &self.cards_vec.len() < &other.cards_vec.len() {
            return Ordering::Greater;
        } else {
            let mut prod1 = 1;
            let mut prod2 = 1;
            for i in 0..self.cards_vec.len() {
                let (_, num1) = &self.cards_vec[i];
                let (_, num2) = other.cards_vec[i];
                prod1 *= num1;
                prod2 *= num2;
            }
            if prod1 == prod2 {
                let order = "23456789TJQKA";
                for i in 0..self.cards_str.len() {
                    let rank1 = order.find(&self.cards_str[i..i + 1]).unwrap();
                    let rank2 = order.find(&other.cards_str[i..i + 1]).unwrap();
                    if rank1 > rank2 {
                        return Ordering::Greater;
                    } else if rank1 < rank2 {
                        return Ordering::Less;
                    }
                }
                return Ordering::Equal;
            } else if prod1 > prod2 {
                return Ordering::Less;
            } else {
                return Ordering::Greater;
            }
        }
    }
}

impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.cmp(other) {
            Ordering::Equal => Some(Ordering::Equal),
            Ordering::Less => Some(Ordering::Less),
            Ordering::Greater => Some(Ordering::Greater),
        }
    }
}
impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        let val = self.partial_cmp(other);
        if val.is_none() {
            return false;
        } else if val.unwrap() == Ordering::Equal {
            return true;
        } else {
            return false;
        }
    }
}

pub fn d7p1() -> u32 {
    let mut d = String::from(env!("CARGO_MANIFEST_DIR"));
    d.push_str("/inputs/input7.txt");
    let file = match File::open(d) {
        Ok(file) => file,
        Err(why) => panic!("{}", why),
    };
    let reader = BufReader::new(file);
    let mut hands: Vec<Hand> = vec![
        Hand {
            cards_str: String::from(""),
            cards_vec: Vec::new(),
            bid: 0,
        };
        1000
    ];
    let mut line_num = 0;
    for line in reader.lines() {
        let text = match line {
            Ok(text) => text,
            Err(why) => panic!("{}", why),
        };
        let split: Vec<&str> = text.split(" ").collect();
        let mut cards_vec: Vec<(String, u32)> = Vec::new();
        let cards_str = String::from(split[0]);
        for i in 0..cards_str.len() {
            let mut added = false;
            let mut j = 0;
            while j < cards_vec.len() && !added {
                let (s, num) = &cards_vec[j];
                if s == &cards_str[i..i + 1] {
                    cards_vec[j] = (s.to_string(), num + 1);
                    added = true;
                }
                j += 1;
            }
            if !added {
                cards_vec.push((cards_str[i..i + 1].to_string(), 1));
            }
        }
        hands[line_num].cards_str = cards_str.clone();
        hands[line_num].cards_vec = cards_vec.clone();
        hands[line_num].bid = split[1].parse::<u32>().unwrap();
        line_num += 1;
    }
    hands.sort();
    let mut res: u32 = 0;
    for i in 0..hands.len() {
        let n: u32 = u32::try_from(i).unwrap();
        res += hands[i].bid * (n + 1);
    }
    res
}

//****************************************************/
#[derive(Clone)]
struct Hand2 {
    cards_str: String,
    cards_vec: Vec<(String, u32)>,
    bid: u32,
}

impl Ord for Hand2 {
    fn cmp(&self, other: &Self) -> Ordering {
        if &self.cards_vec.len() > &other.cards_vec.len() {
            return Ordering::Less;
        } else if &self.cards_vec.len() < &other.cards_vec.len() {
            return Ordering::Greater;
        } else {
            let mut prod1 = 1;
            let mut prod2 = 1;
            for i in 0..self.cards_vec.len() {
                let (_, num1) = &self.cards_vec[i];
                let (_, num2) = other.cards_vec[i];
                prod1 *= num1;
                prod2 *= num2;
            }
            if prod1 == prod2 {
                let order = "J23456789TQKA";
                for i in 0..self.cards_str.len() {
                    let rank1 = order.find(&self.cards_str[i..i + 1]).unwrap();
                    let rank2 = order.find(&other.cards_str[i..i + 1]).unwrap();
                    if rank1 > rank2 {
                        return Ordering::Greater;
                    } else if rank1 < rank2 {
                        return Ordering::Less;
                    }
                }
                return Ordering::Equal;
            } else if prod1 > prod2 {
                return Ordering::Less;
            } else {
                return Ordering::Greater;
            }
        }
    }
}

impl Eq for Hand2 {}

impl PartialOrd for Hand2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.cmp(other) {
            Ordering::Equal => Some(Ordering::Equal),
            Ordering::Less => Some(Ordering::Less),
            Ordering::Greater => Some(Ordering::Greater),
        }
    }
}
impl PartialEq for Hand2 {
    fn eq(&self, other: &Self) -> bool {
        let val = self.partial_cmp(other);
        if val.is_none() {
            return false;
        } else if val.unwrap() == Ordering::Equal {
            return true;
        } else {
            return false;
        }
    }
}

pub fn d7p2() -> u32 {
    let mut d = String::from(env!("CARGO_MANIFEST_DIR"));
    d.push_str("/inputs/input7.txt");
    let file = match File::open(d) {
        Ok(file) => file,
        Err(why) => panic!("{}", why),
    };
    let reader = BufReader::new(file);
    let mut hands: Vec<Hand2> = vec![
        Hand2 {
            cards_str: String::from(""),
            cards_vec: Vec::new(),
            bid: 0,
        };
        1000
    ];
    let mut line_num = 0;
    for line in reader.lines() {
        let text = match line {
            Ok(text) => text,
            Err(why) => panic!("{}", why),
        };
        let split: Vec<&str> = text.split(" ").collect();
        let mut cards_vec: Vec<(String, u32)> = Vec::new();
        let cards_str = String::from(split[0]);
        for i in 0..cards_str.len() {
            let mut added = false;
            let mut j = 0;
            while j < cards_vec.len() && !added {
                let (s, num) = &cards_vec[j];
                if s == &cards_str[i..i + 1] {
                    cards_vec[j] = (s.to_string(), num + 1);
                    added = true;
                }
                j += 1;
            }
            if !added {
                cards_vec.push((cards_str[i..i + 1].to_string(), 1));
            }
        }
        let mut i = 0;
        let mut found_joker = false;
        if cards_vec.len() > 1 {
            while i < cards_vec.len() && !found_joker {
                let (card, count) = &cards_vec[i];
                if card == "J" {
                    let mut max_idx = 0;
                    let mut maximum = 0;
                    let mut j = 0;
                    while j < cards_vec.len() {
                        let (c, cnt) = &cards_vec[j];
                        if cnt > &maximum && c != "J" {
                            maximum = *cnt;
                            max_idx = j;
                        }
                        j += 1;
                    }
                    cards_vec[max_idx] = (String::from(card), count + maximum);
                    cards_vec.remove(i);
                    found_joker = true;
                }
                i += 1;
            }
        }
        hands[line_num].cards_str = cards_str.clone();
        hands[line_num].cards_vec = cards_vec.clone();
        hands[line_num].bid = split[1].parse::<u32>().unwrap();
        line_num += 1;
    }
    hands.sort();
    let mut res: u32 = 0;
    for i in 0..hands.len() {
        let n: u32 = u32::try_from(i).unwrap();
        res += hands[i].bid * (n + 1);
    }
    res
}
