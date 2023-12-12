use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::iter::zip;

fn winning_times(race_time: i64, best_dis: i64) -> i64 {
    let race_time = race_time as f64;
    let best_dis = best_dis as f64;

    let delta = f64::sqrt(race_time * race_time - 4.0 * best_dis);
    let min = 0.5 * (race_time - delta);
    let max = 0.5 * (race_time + delta);

    let mut count = max.floor() - min.ceil() + 1.0;
    if min == min.trunc() { count -= 1.0; }
    if max == max.trunc() { count -= 1.0; }

    count as i64
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <input>", args[0]);
        return;
    }

    let mut file = match File::open(&args[1]) {
        Ok(r) => r,
        Err(e) => { eprintln!("{}", e); return; },
    };

    let mut content = String::new();
    match file.read_to_string(&mut content){
        Ok(r) => r,
        Err(e) => { eprintln!("{}", e); return; },
    };

    let lines: Vec<&str> = content.split('\n').collect();

    let times: Vec<i64> = lines[0]
        .split(":").nth(1).unwrap()
        .replace(" ", "")
        .split(" ").filter_map(|s| s.parse().ok())
        .collect();

    let spaces: Vec<i64> = lines[1]
        .split(":").nth(1).unwrap()
        .replace(" ", "")
        .split(" ").filter_map(|s| s.parse().ok())
        .collect();

    let prod: i64 = zip(&times, &spaces)
        .map(|(t, s)| winning_times(*t, *s))
        .product();
    println!("prod: {prod}");
}
