use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <input>", args[0]);
        return
    }

    let mut file = match File::open(&args[1]) {
        Ok(r) => r,
        Err(e) => { eprintln!("{}", e); return },
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents){
        Ok(r) => r,
        Err(e) => { eprintln!("{}", e); return },
    };

    // captures named groups:
    // - "c", matching the card number
    // - "w", matching winning numbers
    // - "o", matching owned numbers
    let re = Regex::new(r"^Card\s*(?<c>\d+):(?<w>[\s\d]+)\|(?<o>[\s\d]+)$").unwrap();

    let mut card_count = HashMap::new();

    for line in contents.split('\n') {
        let c = match re.captures(line) {
            Some(c) => c,
            None => continue,
        };

        let card: usize = c["c"].parse().unwrap();

        let card_copies = 1 + match card_count.get(&card) {
            Some(n) => *n,
            None => 0
        };
        card_count.insert(card, card_copies);

        let win_nums: HashSet<i32> = c["w"]
            .split(' ')
            .filter_map(|s| s.parse().ok())
            .collect();

        let own_nums: HashSet<i32> = c["o"]
            .split(' ')
            .filter_map(|s| s.parse().ok())
            .collect();

        let matching_numbers = HashSet::intersection(&win_nums, &own_nums).count();

        for next in (card + 1)..(card + matching_numbers + 1) {
            let next_copies = match card_count.get(&next) {
                Some(n) => *n,
                None => 0
            };
            card_count.insert(next, card_copies + next_copies);
        }
    }

    let cards: i32 = card_count.values().sum();
    println!("cards: {:?}", cards);
}
