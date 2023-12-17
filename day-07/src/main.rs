use counter::Counter;
use core::cmp::Ordering;
use std::collections::HashMap;
use std::env;
use std::iter::zip;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard     = 0,
    OnePair      = 1,
    TwoPairs     = 2,
    ThreeOfAKind = 3,
    FullHouse    = 4,
    FourOfAKind  = 5,
    FiveOfAKind  = 6,
}

#[derive(Clone, Debug)]
struct Hand {
    values: Vec<usize>,
    bid: usize,
}

impl Hand {
    fn get_type(&self) -> HandType {
        let counter: Counter<&usize, usize> = self.values.iter().collect();
        let n: usize = counter.len();
        let mc: Vec<(&usize, usize)> = counter.most_common();

        if n == 5                       { return HandType::HighCard; }
        if n == 4                       { return HandType::OnePair; }
        if mc[0].1 == 2 && mc[1].1 == 2 { return HandType::TwoPairs; }
        if mc[0].1 == 3 && mc[1].1 == 1 { return HandType::ThreeOfAKind; }
        if mc[0].1 == 3 && mc[1].1 == 2 { return HandType::FullHouse; }
        if mc[0].1 == 4                 { return HandType::FourOfAKind; }
        if n == 1                       { return HandType::FiveOfAKind; }

        None::<HandType>.expect("Failed to determine hand type: {self}")
    }

    fn improve(&self) -> Hand {
        // no jokers (value 0), cannot improve
        if !self.values.contains(&0) {
            return self.clone();
        }

        let not_j_counter: Counter<&usize, usize> = self.values
            .iter().filter(|v| **v != 0).collect();
        let mc: Vec<(&usize, usize)> = not_j_counter.most_common();

        let joker: usize = match self.get_type() {
            // five jokers
            HandType::FiveOfAKind => 12,
            // extend the highest pair
            HandType::TwoPairs => *mc.iter()
                .filter(|(v, n)| **v != 0 && *n == 2)
                .map(|(v, _)| *v).max().unwrap(),
            // emulate highest card and become a pair
            HandType::HighCard => *self.values.iter().max().unwrap(),
            // extend the best part of current type
            _ => *mc[0].0,
        };

        Hand {
            values: self.values
            .iter()
            .map(|v| if *v == 0 as usize { joker } else { *v })
            .collect(),
            bid: self.bid.clone()
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.improve().get_type() != other.improve().get_type() {
            return self.improve().get_type().cmp(&other.improve().get_type());
        }
        for (s, o) in zip(&self.values, &other.values) {
            if s != o { return s.cmp(&o); }
        }
        Ordering::Equal
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

impl Eq for Hand { }

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

    let card_value: HashMap<char, usize> = "J23456789TQKA"
        .chars().enumerate().map(|(i, c)| (c, i)).collect();

    let mut hands = Vec::<Hand>::new();
    for line in contents.split('\n').filter(|l| l.len() > 0) {
        let parts: Vec<&str> = line.split(' ').collect();
        if parts.len() != 2 {
            continue;
        }

        let values: Vec<usize> = parts[0]
            .chars().map(|c| card_value[&c]).collect();

        let bid: usize = parts[1].parse().unwrap();

        let hand = Hand { values, bid };
        hands.push(hand);
    }

    hands.sort();

    let sum: usize = hands.iter().enumerate()
        .map(|(i, h)| (i + 1) * h.bid)
        .sum();
    println!("winnings: {}", sum);
}
