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
        let chars = line.chars().collect::<Vec<_>>();

        let first: u32 = match line.find(|c: char| c.is_digit(10)) {
            Some(r) => chars[r].to_digit(10).unwrap(),
            None => continue,
        };
        let last: u32 = chars[
            line.rfind(|c: char| c.is_digit(10)).unwrap()
        ].to_digit(10).unwrap();

        sum += first * 10 + last;
    }

    println!("sum: {:?} ", sum);
}
