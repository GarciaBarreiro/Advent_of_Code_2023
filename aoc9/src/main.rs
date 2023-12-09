use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    stars();
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn predict_next(x: &Vec<i64>) -> i64 {
    let mut diffs: Vec<i64> = Vec::new();
    let mut a = x[0];
    let mut all_zero = true;
    let mut i = 0;
    for &b in x {
        if i != 0 {
            diffs.push(b - a);
            if all_zero && b - a != 0 { all_zero = false; }
            a = b;
        }
        i+=1;   // don't like this
    }

    let mut ret = 0;
    if let Some(b) = x.last() {
        if !all_zero {
            ret = b + predict_next(&diffs);
        } else {
            ret = *b;
        }
    }
    ret
}

fn predict_last(x: &Vec<i64>) -> i64 {
    let mut diffs: Vec<i64> = Vec::new();
    let mut a = x[0];
    let mut all_zero = true;
    let mut i = 0;
    for &b in x {
        if i != 0 {
            diffs.push(b - a);
            if all_zero && b - a != 0 { all_zero = false; }
            a = b;
        }
        i += 1;
    }

    if !all_zero {
        x[0] - predict_last(&diffs)
    } else {
        x[0]
    }
}

fn stars() {
    let mut res_next = 0;
    let mut res_last = 0;
    if let Ok(lines) = read_lines("./input") {
        for line in lines {
            if let Ok(l) = line {
                let split: Vec<i64> = l.split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect();
                res_next += predict_next(&split);
                res_last += predict_last(&split);
            }
        }
    }
    println!("one star sol = {}", res_next);
    println!("two star sol = {}", res_last);
}
