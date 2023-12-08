use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;
use regex::Regex;

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
    let mut nodes = HashMap::new();
    let mut directions = "".to_string();
    let re = Regex::new(r"(\w{3})").unwrap();
    if let Ok(lines) = read_lines("./input") {
        let mut i = 0;
        for line in lines {
            if let Ok(l) = line {
                if i < 2 {
                    if i == 0 { directions = l.clone(); }
                } else {
                    let mut v: [String;3] = ["".to_string(),"".to_string(),"".to_string()];
                    let mut j = 0;
                    for split in re.captures_iter(&l) {
                        let _ = &split[1].clone_into(&mut v[j]);
                        j += 1;
                    }
                    // println!("{:?}", v);
                    nodes.insert(v[0].clone(), [v[1].clone(),v[2].clone()]);
                }
            }
            i += 1;
        }
    }
    let mut current = "AAA".to_string();
    let finish = "ZZZ".to_string();
    let mut steps = 0;
    let mut end = false;
    while !end {
        for c in directions.chars() {
            let pos = if c == 'L' { 0 } else { 1 };
            current = nodes[&current][pos].clone();
            if current.eq(&finish) { end = true; }
            steps += 1;
        }
    }
    println!("one star sol = {}", steps);
}

fn gcd(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn lcm(a: u128, b: u128) -> u128 {
    (a * b) / gcd(a,b)
}

fn second_star() {
    let mut nodes = HashMap::new();
    let mut directions = "".to_string();
    let re = Regex::new(r"(\w{3})").unwrap();
    let mut starts = Vec::new();
    if let Ok(lines) = read_lines("./input") {
        let mut i = 0;
        for line in lines {
            if let Ok(l) = line {
                if i < 2 {
                    if i == 0 { directions = l.clone(); }
                } else {
                    let mut v: [String;3] = ["".to_string(),"".to_string(),"".to_string()];
                    let mut j = 0;
                    for split in re.captures_iter(&l) {
                        let _ = &split[1].clone_into(&mut v[j]);
                        j += 1;
                    }
                    nodes.insert(v[0].clone(), [v[1].clone(),v[2].clone()]);
                    if v[0].char_indices().nth_back(0).unwrap().1 == 'A' { starts.push(v[0].clone()); }
                }
            }
            i += 1;
        }
    }
    
    let mut tot_steps = Vec::new();
    for sta in &starts{
        let mut end = false;
        let mut current = sta.to_string();
        let mut steps = 0;
        while !end {
            for c in directions.chars() {
                let pos = if c == 'L' { 0 } else { 1 };
                current = nodes[&current][pos].clone();
                if current.char_indices().nth_back(0).unwrap().1 == 'Z' { end = true; }
                steps += 1;
            }
        }
        tot_steps.push(steps);
    }

    let mut res = tot_steps[0];
    for st in tot_steps {
        res = lcm(res, st);
    }

    println!("two star sol = {}", res);
}
