use std::collections::HashMap;
use std::collections::VecDeque;
use std::env;
use std::fs::File;
use std::io::prelude::*;

#[derive(Clone)]
#[derive(Debug)]
struct AlmanacRange {
    start: i64,
    count: i64
}

impl AlmanacRange {
    fn split(&self, left_cut: i64, right_cut: i64) ->
            (Option<AlmanacRange>, Option<AlmanacRange>, Option<AlmanacRange>) {
        let lstart = self.start;
        let lstop = i64::min(left_cut, self.start + self.count);
        let lcount = i64::max(0, lstop - lstart);

        let cstart = i64::max(left_cut, self.start);
        let cstop = i64::min(right_cut, self.start + self.count);
        let ccount = i64::max(0, cstop - cstart);

        let rstart = i64::max(right_cut, self.start);
        let rstop = self.start + self.count;
        let rcount = i64::max(0, rstop - rstart);

        assert_eq!(self.count, lcount + ccount + rcount);
        (
            if lcount > 0 { Some(AlmanacRange { start: lstart, count: lcount }) } else { None },
            if ccount > 0 { Some(AlmanacRange { start: cstart, count: ccount }) } else { None },
            if rcount > 0 { Some(AlmanacRange { start: rstart, count: rcount }) } else { None },
        )
    }
}

fn parse_seed_ranges(value: &str) -> Option<Vec<AlmanacRange>> {
    match value.strip_prefix("seeds: ") {
        Some(s) => Some(s
            .split(' ')
            .filter_map(|s| s.parse().ok())
            .collect::<Vec<i64>>()
            .chunks(2)
            .map(|c| AlmanacRange { start: c[0], count: c[1] })
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

    fn apply(&self, src_ranges: Vec<AlmanacRange>) -> Vec<AlmanacRange> {
        let mut dst_ranges: Vec<AlmanacRange> = vec![];
        let mut src_ranges = VecDeque::from(src_ranges);

        for mr in &self.ranges {
            let (lcut, rcut) = (mr.src_start, mr.src_start + mr.count);

            let mut unmapped: Vec<AlmanacRange> = vec![];
            while let Some(sr) = src_ranges.pop_front() {
                // Use map range boundaries to cut the source range:
                // the center part does overlap with it, so must be
                // mapped, while left and right parts won't change.
                let (left, center, right) = sr.split(lcut, rcut);

                if let Some(r) = center {
                    let mapped = AlmanacRange {
                        start: r.start + mr.dst_start - mr.src_start,
                        count: r.count
                    };
                    dst_ranges.push(mapped);
                }

                if let Some(r) = left {
                    unmapped.push(r);
                }

                if let Some(r) = right {
                    unmapped.push(r);
                }
            }

            // Source ranges (or their parts) which could not be mapped
            // through the current map range may still be successfully
            // mapped with the next one, so put them back into the queue.
            src_ranges.extend(unmapped);
        }

        // The ranges (or their parts) still in the queue, could not
        // be mapped by any map range, and will remain unchanged.
        dst_ranges.extend(src_ranges);

        dst_ranges
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

    fn map(&self, src: &str, dst: &str, src_range: AlmanacRange) -> Option<Vec<AlmanacRange>> {
        let mut ids = vec![src_range.clone()];
        let mut curr = src.to_owned();

        while &curr != &dst {
            let next = self.src_dst.get(&curr).unwrap().to_owned();

            let src_dst = &(curr.to_owned(), next.to_owned());
            let map = self.src_dst_map.get(src_dst).unwrap();

            ids = map.apply(ids);
            curr = next;
        }
        Some(ids)
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

    let mut seed_ranges: Vec<AlmanacRange> = vec![];
    let mut almanac = Almanac::new();

    for (i, section) in content.split("\n\n").enumerate() {
        if section.starts_with("seeds: ") {
            match parse_seed_ranges(section) {
                Some(v) => seed_ranges = v,
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

    let mapped_seed_ranges: Vec<AlmanacRange> = seed_ranges.iter()
        .filter_map(|s| almanac.map("seed", "location", s.clone()))
        .flatten()
        .collect();

    let min_loc: i64 = mapped_seed_ranges
        .iter()
        .map(|r| r.start)
        .min()
        .unwrap();
    println!("Lowest location: {:?}", min_loc);

}
