use regex::Regex;
use std::env;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
struct Layer {
    m: Vec<Vec<bool>>
}

#[derive(Debug)]
struct Token<T> {
    tok: T,
    x0: usize, x1: usize, y: usize,
}
type Num = Token<i32>;
type Sym = Token<char>;

trait FromMatch {
    fn from_match(m: regex::Match, y: usize) -> Self;
}

impl FromMatch for Num {
    fn from_match(m: regex::Match, y: usize) -> Self {
        let tok = m.as_str().parse::<i32>().unwrap();
        let x0 = m.start();
        let x1 = m.end();
        Num { tok, x0, x1, y }
    }
}

impl FromMatch for Sym {
    fn from_match(m: regex::Match, y: usize) -> Self {
        let tok = m.as_str().chars().nth(0).unwrap();
        let x0 = m.start();
        let x1 = m.end();
        Sym { tok, x0, x1, y }
    }
}

#[derive(Debug)]
struct Schematic {
    numbers: Vec<Num>,
    symbol_layer: Layer,
}

#[derive(Debug)]
struct Rectangle {
    tl_x: usize, tl_y: usize,
    br_x: usize, br_y: usize,
}

impl Rectangle {
    fn to_coords_yx(&self) -> Vec<(usize, usize)> {
        let mut v = vec![];
        for j in self.tl_y..self.br_y {
            for i in self.tl_x..self.br_x {
                v.push((j, i));
            }
        }
        v
    }
}

impl Schematic {
    fn covered_rectangle(&self, n: &Num) -> Rectangle {
        let x_min = if n.x0 == 0 { 0 } else { n.x0 - 1 };
        let x_max = usize::min(n.x1 + 1, self.symbol_layer.m[0].len());
        let y_min = if n.y == 0 { 0 } else { n.y - 1 };
        let y_max = usize::min(n.y + 2, self.symbol_layer.m.len());
        Rectangle { tl_x: x_min, tl_y: y_min, br_x: x_max, br_y: y_max }
    }

    fn adjacent_to_symbol(&self, n: &Num) -> bool {
        let rect = self.covered_rectangle(n);
        rect.to_coords_yx().iter().any(|(j, i)|
            self.symbol_layer.m[*j][*i]
        )
    }

    fn get_part_numbers(&self) -> impl Iterator<Item = &Num> {
        self.numbers.iter().filter(|n|
            self.adjacent_to_symbol(n)
        ).into_iter()
    }
}

fn get_tokens<T: FromMatch>(re: &Regex, y: usize, line: &str) -> Vec<T> {
    re.captures_iter(line).map(|c| {
        let m = c.get(0).unwrap();
        T::from_match(m, y)
    }).collect()
}

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

    let re_num = Regex::new(r"\d+").unwrap();
    let re_sym = Regex::new(r"[^\.\d]").unwrap();

    let mut numbers: Vec<Num> = vec![];
    let mut symbol_layer = Layer { m: vec![] };

    for (j, line) in contents.split('\n').into_iter().enumerate() {
        if line.len() == 0 {
            continue;
        }

        let nums: Vec<Num> = get_tokens(&re_num, j, &line);
        let syms: Vec<Sym> = get_tokens(&re_sym, j, &line);
        //println!("{}  {} nums, {} syms", line, nums.len(), syms.len());

        let mut nmask = vec![false; line.len()];
        for t in &nums {
            for i in t.x0..t.x1 {
                nmask[i] = true;
            }
        }

        let mut smask = vec![false; line.len()];
        for t in &syms {
            for i in t.x0..t.x1 {
                smask[i] = true;
                assert!(!nmask[i], "[{}, {}] was number; ambiguous {:?}", j, i, t);
            }
        }

        numbers.extend(nums);
        symbol_layer.m.push(smask);
    }

    let schematic = Schematic { numbers, symbol_layer };

    let sum: i32 = schematic
        .get_part_numbers()
        .map(|p| p.tok)
        .sum();

    println!("sum: {:?}", sum);
}
