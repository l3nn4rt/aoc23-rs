use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn parse_seeds(value: &str) -> Option<Vec<i64>> {
    match value.strip_prefix("seeds: ") {
        Some(s) => Some(s
            .split(' ')
            .filter_map(|s| s.parse().ok())
            .collect()),
        None => None
    }
}

#[derive(Debug)]
struct AlmanacMapRange {
    src_start: i64,
    dst_start: i64,
    count: i64
}

#[derive(Debug)]
struct AlmanacMap {
    ranges: Vec<AlmanacMapRange>,
}

impl AlmanacMap {
    fn new() -> Self {
        Self { ranges: vec![] }
    }

    fn add_range(&mut self, offset: AlmanacMapRange) {
        let _ = &self.ranges.push(offset);
    }

    fn apply(&self, src_id: i64) -> i64 {
        for range in &self.ranges {
            let within = range.src_start <= src_id && src_id < range.src_start + range.count;
            if within {
                return src_id + range.dst_start - range.src_start;
            }
        }
        src_id
    }
}

fn parse_map_categories(value: &str) -> Option<(String, String)> {
    let lines: Vec<&str> = value.trim().split('\n').collect();
    if lines.len() == 0 { return None; };

    let header = lines[0];
    if !header.ends_with(" map:") { return None; }

    let tokens: Vec<&str> = header
        .split(' ').nth(0).unwrap()
        .split('-').collect();
    if tokens.len() != 3 { return None; }

    let src = tokens[0].to_owned();
    let dst = tokens[2].to_owned();

    Some((src, dst))
}

fn parse_map_ranges(value: &str) -> Option<AlmanacMap> {
    let lines: Vec<&str> = value.trim().split('\n').collect();
    if lines.len() == 0 { return None; };

    let mut map = AlmanacMap::new();
    for line in lines.iter().skip(1) {
        let nums: Vec<i64> = line
            .split(' ')
            .filter_map(|s| s.parse().ok())
            .collect();
        assert_eq!(nums.len(), 3, "Failed parsing map:\n\n{}\n\n", value);
        let dst_start = nums[0];
        let src_start = nums[1];
        let count = nums[2];

        map.add_range(AlmanacMapRange { src_start, dst_start, count });
    }

    Some(map)
}

struct Almanac {
    src_dst: HashMap<String, String>,
    src_dst_map: HashMap<(String, String), AlmanacMap>,
}

impl Almanac {
    fn new() -> Self {
        Almanac {
            src_dst: HashMap::new(),
            src_dst_map: HashMap::new(),
        }
    }

    fn add_map(&mut self, src: String, dst: String, map: AlmanacMap) {
        self.src_dst.insert(src.to_owned(), dst.to_owned());
        self.src_dst_map.insert((src.to_owned(), dst.to_owned()), map);
    }

    fn map(&self, src: &str, dst: &str, src_id: i64) -> Option<i64> {
        let mut id = src_id.clone();
        let mut curr = src.to_owned();

        while &curr != &dst {
            let next = self.src_dst.get(&curr).unwrap().to_owned();

            let src_dst = &(curr.to_owned(), next.to_owned());
            let map = self.src_dst_map.get(src_dst).unwrap();

            id = map.apply(id);
            curr = next;
        }
        Some(id)
    }
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

    let mut seeds: Vec<i64> = vec![];
    let mut almanac = Almanac::new();

    for (i, section) in content.split("\n\n").enumerate() {
        if section.starts_with("seeds: ") {
            match parse_seeds(section) {
                Some(v) => seeds = v,
                None => ()
            };
        } else if section.split('\n').nth(0).unwrap().ends_with(" map:") {
            let (src, dst) = parse_map_categories(section)
                .expect("Failed to parse map categories");
            let map = parse_map_ranges(section)
                .expect("Failed to parse map ranges");
            almanac.add_map(src, dst, map);
        } else {
            assert!(false, "Failed to parse section {i}");
        }
    }

    let min_loc = seeds.iter()
        .filter_map(|s| almanac.map("seed", "location", *s))
        .min().unwrap();
    println!("Lowest location: {min_loc}");

}
