use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Hash, Eq, PartialEq)]
enum Turn {
    Left,
    Right,
}

#[derive(Debug)]
struct Direction<'a> {
    turns: Vec<&'a Turn>,
}

impl Direction<'_> {
    fn new(chars: Vec<char>) -> Self {
        let turns = chars.iter().map(|c| match c {
            'L' => &Turn::Left,
            'R' => &Turn::Right,
            _ => None.expect(&format!("Failed to parse turn from char {}", c))
        }).collect();

        Self { turns }
    }
}

#[derive(Debug)]
struct Graph<'a> {
    nodes: HashMap<(&'a str, &'a Turn), &'a str>,
}

impl<'a> Graph<'a> {
    fn new(lines: &'a [&str]) -> Graph<'a> {
        let re = Regex::new(r"^(?<src>\w*) = \((?<left>\w*), (?<right>\w*)\)$").unwrap();

        let mut nodes = HashMap::<(&str, &'a Turn), &str>::new();
        for line in lines {
            if let Some(c) = re.captures(line) {
                let src = c.name("src").unwrap().as_str();
                let left = c.name("left").unwrap().as_str();
                let right = c.name("right").unwrap().as_str();
                nodes.insert((src, &Turn::Left), left);
                nodes.insert((src, &Turn::Right), right);
            }
        }

        Graph { nodes }
    }

    fn path(&self, start: &'a str, end: &'a str, direction: &Direction) -> Vec<&'a str> {
        let mut curr = start;
        let mut path = vec![];

        for turn in direction.turns.iter().cycle() {
            curr = self.nodes
                .get(&(curr, *turn))
                .expect(&format!("No transition from <{:?}, {:?}>", curr, turn));
            path.push(curr);
            if curr == end { break }
        }

        path
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
        Err(e) => { eprintln!("{}", e); return },
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents){
        Ok(r) => r,
        Err(e) => { eprintln!("{}", e); return },
    };

    let lines: Vec<&str> = contents
        .split('\n')
        .filter(|l| l.len() > 0)
        .collect();

    let direction = Direction::new(lines[0].chars().collect());
    let graph = Graph::new(&lines[1..]);

    let path = graph.path("AAA", "ZZZ", &direction);
    println!("steps: {:?}", path.len());

}
