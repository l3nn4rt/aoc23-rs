use regex::Regex;
use std::env;
use std::fs::File;
use std::io::prelude::*;

static MAX_RED:   i32 = 12;
static MAX_GREEN: i32 = 13;
static MAX_BLUE:  i32 = 14;

#[derive(Debug)]
struct Hand {
    red:   i32,
    green: i32,
    blue:  i32,
}

impl Hand {
    fn is_possible(&self) -> bool {
        self.red <= MAX_RED && self.green <= MAX_GREEN && self.blue <= MAX_BLUE
    }

    fn collect_from_game(re_hand: &Regex, line: &str) -> Vec<Hand> {
        re_hand.captures_iter(line).map(|c| {
            let r = match c.name("red") {
                Some(m) => m.as_str().parse::<i32>().unwrap(),
                None => 0
            };
            let g = match c.name("green") {
                Some(m) => m.as_str().parse::<i32>().unwrap(),
                None => 0
            };
            let b = match c.name("blue") {
                Some(m) => m.as_str().parse::<i32>().unwrap(),
                None => 0
            };
            Hand { red: r, green: g, blue: b }
        }).collect()
    }

    fn get_requirements(hands: &Vec<Hand>) -> Hand {
        let mut required = Hand { red: 0, green: 0, blue: 0 };
        for hand in hands {
            required.red   = i32::max(hand.red,   required.red);
            required.green = i32::max(hand.green, required.green);
            required.blue  = i32::max(hand.blue,  required.blue);
        }
        required
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <input>", args[0]);
        return
    }

    let mut file = match File::open(&args[1]) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("{}", e);
            return
        },
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents){
        Ok(r) => r,
        Err(e) => {
            eprintln!("{}", e);
            return
        },
    };

    let re_id = Regex::new(r"^Game (?<id>\d+):").unwrap();
    let _r = r"\s*((?<red>\d+) red)\s*";
    let _g = r"\s*((?<green>\d+) green)\s*";
    let _b = r"\s*((?<blue>\d+) blue)\s*";
    let re_hand = Regex::new(format!(
        r"(?<hand>({}|{}|{}),?)+(;|$)", _r, _g, _b).as_str()).unwrap();

    let mut sum: i32 = 0;
    let mut power_sum: i32 = 0;
    for line in contents.split('\n') {
        let id = match re_id.captures(line) {
            Some(r) => r["id"].parse::<i32>().unwrap(),
            None => continue,
        };

        let hands = Hand::collect_from_game(&re_hand, line);
        if hands.iter().all(|h| h.is_possible()) {
            sum += id;
        }

        let req = Hand::get_requirements(&hands);
        power_sum += req.red * req.green * req.blue;
    }

    println!("sum: {:?} ", sum);
    println!("power sum: {:?} ", power_sum);
}
