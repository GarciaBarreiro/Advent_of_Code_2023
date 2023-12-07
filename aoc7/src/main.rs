use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::cmp::Ordering;

#[derive(PartialEq, PartialOrd, Default, Eq, Clone, Copy)]
enum Cards1 {
    #[default]
    Def,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    J,
    Q,
    K,
    A,
}

#[derive(PartialEq, PartialOrd, Default, Eq, Clone, Copy)]
enum Cards2 {
    #[default]
    J,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    Q,
    K,
    A,
}

#[derive(PartialEq, PartialOrd, Default, Eq, Clone, Copy)]
enum Tp {
    #[default]
    High,
    One,
    Two,
    Three,
    Full,
    Four,
    Five,
}

#[derive(Default, PartialEq, Eq, Clone, Copy)]
struct Hand1 {
    hand: [Cards1;5],   // full hand
    bid: u32,           // bid
    tp: Tp,             // type of hand
}

impl Hand1 {
    fn is_better_than(&self, h: Hand1) -> Ordering {
        if self.tp > h.tp { return Ordering::Greater; }
        else if self.tp < h.tp { return Ordering::Less; }
        for i in 0..self.hand.len() {
            if self.hand[i] > h.hand[i] { return Ordering::Greater; }
            else if self.hand[i] < h.hand[i] { return Ordering::Less; }
        }
        Ordering::Equal
    }
}

impl PartialOrd for Hand1 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.is_better_than(*other))
    }
}

impl Ord for Hand1 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.is_better_than(*other)
    }
}

#[derive(Default, PartialEq, Eq, Clone, Copy)]
struct Hand2 {
    hand: [Cards2;5],   // full hand
    bid: u32,           // bid
    tp: Tp,             // type of hand
}

impl Hand2 {
    fn is_better_than(&self, h: Hand2) -> Ordering {
        if self.tp > h.tp { return Ordering::Greater; }
        else if self.tp < h.tp { return Ordering::Less; }
        for i in 0..self.hand.len() {
            if self.hand[i] > h.hand[i] { return Ordering::Greater; }
            else if self.hand[i] < h.hand[i] { return Ordering::Less; }
        }
        Ordering::Equal
    }
}

impl PartialOrd for Hand2 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.is_better_than(*other))
    }
}

impl Ord for Hand2 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.is_better_than(*other)
    }
}

fn main() {
    first_star();
    second_star();
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn first_star() {
    let mut hands: Vec<Hand1> = Vec::new();
    if let Ok(lines) = read_lines("./input") {
        for line in lines {
            if let Ok(l) = line {
                let split: Vec<String> = l.split_whitespace().map(str::to_string).collect();
                let mut h = Hand1::default();
                let mut i = 0;
                let mut diff_cards: [u8;13] = [0;13];
                for c in split[0].chars() {
                    match c {
                        '2' => { h.hand[i] = Cards1::Two; diff_cards[0] += 1 },
                        '3' => { h.hand[i] = Cards1::Three; diff_cards[1] += 1 },
                        '4' => { h.hand[i] = Cards1::Four; diff_cards[2] += 1 },
                        '5' => { h.hand[i] = Cards1::Five; diff_cards[3] += 1 },
                        '6' => { h.hand[i] = Cards1::Six; diff_cards[4] += 1 },
                        '7' => { h.hand[i] = Cards1::Seven; diff_cards[5] += 1 },
                        '8' => { h.hand[i] = Cards1::Eight; diff_cards[6] += 1 },
                        '9' => { h.hand[i] = Cards1::Nine; diff_cards[7] += 1 },
                        'T' => { h.hand[i] = Cards1::T; diff_cards[8] += 1 },
                        'J' => { h.hand[i] = Cards1::J; diff_cards[9] += 1 },
                        'Q' => { h.hand[i] = Cards1::Q; diff_cards[10] += 1 },
                        'K' => { h.hand[i] = Cards1::K; diff_cards[11] += 1 },
                        'A' => { h.hand[i] = Cards1::A; diff_cards[12] += 1 },
                        _ => println!("What"),
                    }
                    i += 1;
                }
                let mut trio = false;
                let mut pair = false;
                for val in diff_cards {
                    if val == 5 { h.tp = Tp::Five; break; }
                    if val == 4 { h.tp = Tp::Four; break; }
                    if val == 3 {
                        if pair { h.tp = Tp::Full; break; }
                        else { trio = true; }
                    }
                    if val == 2 {
                        if pair { h.tp = Tp::Two; break; }
                        else { pair = true; }
                        if trio { h.tp = Tp::Full; break; }
                    }
                }

                if h.tp == Tp::default() {
                    if pair { h.tp = Tp::One; }
                    if trio { h.tp = Tp::Three; }
                }

                h.bid = split[1].to_string().parse::<u32>().unwrap();
                hands.push(h);
            }
        }
    }
    hands.sort();
    let mut winnings = 0;
    for i in 1..=hands.len() {
        println!("{},{}", i, hands[i-1].bid);
        winnings += hands[i-1].bid as usize * i;
    }
    println!("one star sol = {}", winnings);
}

fn second_star() {
    let mut hands: Vec<Hand2> = Vec::new();
    if let Ok(lines) = read_lines("./input") {
        for line in lines {
            if let Ok(l) = line {
                let split: Vec<String> = l.split_whitespace().map(str::to_string).collect();
                let mut h = Hand2::default();
                let mut i = 0;
                let mut diff_cards: [u8;13] = [0;13];
                for c in split[0].chars() {
                    match c {
                        '2' => { h.hand[i] = Cards2::Two; diff_cards[0] += 1 },
                        '3' => { h.hand[i] = Cards2::Three; diff_cards[1] += 1 },
                        '4' => { h.hand[i] = Cards2::Four; diff_cards[2] += 1 },
                        '5' => { h.hand[i] = Cards2::Five; diff_cards[3] += 1 },
                        '6' => { h.hand[i] = Cards2::Six; diff_cards[4] += 1 },
                        '7' => { h.hand[i] = Cards2::Seven; diff_cards[5] += 1 },
                        '8' => { h.hand[i] = Cards2::Eight; diff_cards[6] += 1 },
                        '9' => { h.hand[i] = Cards2::Nine; diff_cards[7] += 1 },
                        'T' => { h.hand[i] = Cards2::T; diff_cards[8] += 1 },
                        'J' => { h.hand[i] = Cards2::J; diff_cards[9] += 1 },
                        'Q' => { h.hand[i] = Cards2::Q; diff_cards[10] += 1 },
                        'K' => { h.hand[i] = Cards2::K; diff_cards[11] += 1 },
                        'A' => { h.hand[i] = Cards2::A; diff_cards[12] += 1 },
                        _ => println!("What"),
                    }
                    i += 1;
                }
                let mut five = false;
                let mut four = false;
                let mut full = false;
                let mut trio = false;
                let mut two_pair = false;
                let mut one_pair = false;
                for val in diff_cards {
                    if val == 5 { five = true; break; }
                    if val == 4 { four = true; break; }
                    if val == 3 {
                        if one_pair { full = true; break; }
                        else { trio = true; }
                    }
                    if val == 2 {
                        if one_pair { two_pair = true; break; }
                        else { one_pair = true; }
                        if trio { full = true; break; }
                    }
                }

                if diff_cards[9] > 0 {
                    if four { h.tp = Tp::Five; }        // either J is 4 or the other card, we don't care
                    else if full { h.tp = Tp::Five; }   // either J is 2 or the other card, we don't care
                    else if trio { h.tp = Tp::Four; }
                    else if two_pair {
                        if diff_cards[9] == 1 { h.tp = Tp::Full; }  // 2 of one, 3 of the other
                        else { h.tp = Tp::Four; }
                    }
                    else if one_pair { h.tp = Tp::Three; }
                    else { h.tp = Tp::One; }
                } else {
                    if four { h.tp = Tp::Four; }
                    else if full { h.tp = Tp::Full; }
                    else if trio { h.tp = Tp::Three; }
                    else if two_pair { h.tp = Tp::Two; }
                    else if one_pair { h.tp = Tp::One; }
                }

                if five { h.tp = Tp::Five; }

                h.bid = split[1].to_string().parse::<u32>().unwrap();
                hands.push(h);
            }
        }
    }
    hands.sort();
    let mut winnings = 0;
    for i in 1..=hands.len() {
        println!("{}", hands[i-1].bid);
        winnings += hands[i-1].bid as usize * i;
    }
    println!("two star sol = {}", winnings);
}
