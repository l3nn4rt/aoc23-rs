use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn replace_leftmost<'a>(line: &'a str, trans: &HashMap<&'a str, &'a str>) -> String {
    let mut running_idx: usize = line.len();
    let mut running_line: String = line.to_string();
    for (old, new) in trans {
        match line.find(old) {
            Some(r) => {
                if r < running_idx {
                    running_idx = r;
                    running_line = line.replacen(old, new, 1).to_string();
                }
            },
            None => continue,
        };
    };
    running_line
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let ltrans: HashMap<&str, &str> = HashMap::from([
        ("zero", "0"), ("one", "1"), ("two", "2"), ("three", "3"), ("four", "4"),
        ("five", "5"), ("six", "6"), ("seven", "7"), ("eight", "8"), ("nine", "9"),
    ]);

    let _rtrans:HashMap::<String, &str> = HashMap::from_iter(ltrans.iter().map(
        |(key, value)| { (key.chars().rev().collect::<String>(), *value) }
    ));
    let rtrans: HashMap<&str, &str> = HashMap::from_iter(_rtrans.iter().map(
        |(key, value)| { (key.as_str(), *value) }
    ));

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

    let mut sum: u32 = 0;
    for line in contents.split('\n'){

        let replaced_leftmost = replace_leftmost(line, &ltrans);

        let first: u32 = match replaced_leftmost.find(|c: char| c.is_digit(10)) {
            Some(r) => {
                let chars = replaced_leftmost.chars().collect::<Vec<_>>();
                chars[r].to_digit(10).unwrap()
            },
            None => continue,
        };

        let rev_line = line.chars().rev().collect::<String>();
        let rev_repl = replace_leftmost(rev_line.as_str(), &rtrans);
        let replaced_rightmost = rev_repl.chars().rev().collect::<String>();

        let last: u32 = match replaced_rightmost.rfind(|c: char| c.is_digit(10)) {
            Some(r) => {
                let chars = replaced_rightmost.chars().collect::<Vec<_>>();
                chars[r].to_digit(10).unwrap()
            },
            None => continue,
        };

        sum += first * 10 + last;
    }

    println!("sum: {:?} ", sum);
}
