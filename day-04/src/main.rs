use regex::Regex;
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
    // - "w", matching winning numbers
    // - "o", matching owned numbers
    let re = Regex::new(r"^Card\s*\d+:(?<w>[\s\d]+)\|(?<o>[\s\d]+)$").unwrap();

    let mut sum: usize = 0;
    for line in contents.split('\n') {
        let c = match re.captures(line) {
            Some(c) => c,
            None => continue,
        };

        let win_nums: HashSet<i32> = c["w"]
            .split(' ')
            .filter_map(|s| s.parse().ok())
            .collect();

        let own_nums: HashSet<i32> = c["o"]
            .split(' ')
            .filter_map(|s| s.parse().ok())
            .collect();

        let count = HashSet::intersection(&win_nums, &own_nums).count();
        let value = if count > 0 { 1 << (count - 1) } else { 0 };

        sum += value;
    }

    println!("sum: {:?}", sum);
}
