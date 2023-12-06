use regex::Regex;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
struct Layer<T> {
    m: Vec<Vec<T>>
}

#[derive(Clone)]
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
    gears: Vec<Sym>,
    numbers: Vec<Num>,
    number_layer: Layer<Option<usize>>,
    symbol_layer: Layer<bool>,
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
    fn covered_rectangle<T>(&self, n: &Token<T>) -> Rectangle {
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

    fn get_numbers_on_rectangle(&self, rect: Rectangle) -> Vec<&Num> {
        let mut s = HashSet::new();
        let mut v = vec![];

        for (y, x) in rect.to_coords_yx() {
            match self.number_layer.m[y][x] {
                Some(idx) => {
                    if !s.contains(&idx) {
                        s.insert(idx);
                        v.push(&self.numbers[idx]);
                    }
                },
                None => continue
            };
        }
        v
    }

    fn get_adjacent_numbers(&self, s: &Sym) -> Vec<&Num> {
        let rect = self.covered_rectangle(s);
        self.get_numbers_on_rectangle(rect)
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

    let mut gears: Vec<Sym> = vec![];
    let mut numbers: Vec<Num> = vec![];
    let mut number_layer = Layer { m: vec![] };
    let mut symbol_layer = Layer { m: vec![] };

    for (j, line) in contents.split('\n').into_iter().enumerate() {
        if line.len() == 0 {
            continue;
        }

        let nums: Vec<Num> = get_tokens(&re_num, j, &line);
        let syms: Vec<Sym> = get_tokens(&re_sym, j, &line);
        //println!("{}  {} nums, {} syms", line, nums.len(), syms.len());

        let numbers_count = numbers.len();
        let mut nmask = vec![std::option::Option::None; line.len()];
        for (k, t) in nums.iter().enumerate() {
            for i in t.x0..t.x1 {
                let idx = numbers_count + k;
                nmask[i] = Some(idx);
            }
        }

        let mut smask = vec![false; line.len()];
        for t in &syms {
            for i in t.x0..t.x1 {
                smask[i] = true;
                assert!(match nmask[i] {
                    Some(_) => false,
                    None => true
                } , "[{}, {}] was number; ambiguous {:?}", j, i, t);
            }
            if t.tok == '*' {
                gears.push(t.to_owned());
            }
        }

        numbers.extend(nums);
        number_layer.m.push(nmask);
        symbol_layer.m.push(smask);
    }

    let schematic = Schematic { gears, numbers, number_layer, symbol_layer };

    let pn_sum: i32 = schematic
        .get_part_numbers()
        .map(|p| p.tok)
        .sum();

    let gear_sum: i32 = schematic.gears.iter()
        .map(|g| schematic.get_adjacent_numbers(g))
        .filter(|ns| ns.len() == 2)
        .map(|ns| ns[0].tok * ns[1].tok)
        .sum();

    println!("part numbers sum: {:?}", pn_sum);
    println!("gear ratios sum: {:?}", gear_sum);
}
