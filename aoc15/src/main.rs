use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Clone, Debug)]
struct Boxes {
    label: String,
    lens: usize,
}

impl Boxes {
    fn is_eq(&self, other: &Self) -> bool {
        self.label.eq(&other.label)
    }
}

fn main() {
    one_star_sol("./input");
    two_star_sol("./input");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn inp(input: &str) -> Vec<String> {
    let mut ch: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines(input) {
        for line in lines {
            if let Ok(l) = line {
                ch = l.split(',').map(|s| s.to_string()).collect();
            }
        }
    }
    ch
}

fn hash(s: String) -> usize {
    let mut sum = 0;
    for c in s.chars() {
        sum += c as usize;
        sum *= 17;
        sum %= 256;
    }
    sum
}

fn one_star_sol(input: &str) {
    let ch = inp(input);
  
    let mut tot = 0;
    for s in ch {
        tot += hash(s);
    }
    println!("one star sol: {}", tot);
}

fn two_star_sol(input: &str) {
    let ch = inp(input);

    let mut boxes: Vec<Vec<Boxes>> = std::iter::repeat(vec![]).take(256).collect();

    for s in ch {
        if s.contains('=') {
            let parts: Vec<String> = s.split('=').map(|s| s.to_string()).collect();
            let label = hash(parts[0].clone());
            let lens = parts[1].parse::<usize>().unwrap();
            let mut cube = boxes[label].clone();    // box is a reserved word :(
            let b = Boxes { label: parts[0].clone(), lens };
            if cube.len() == 0 {
                cube.push(b);
            } else {
                let mut found = false;
                for i in 0..cube.len() {
                    if cube[i].is_eq(&b) {
                        cube[i] = b.clone();
                        found = true;
                        break;
                    }
                }
                if !found {
                    cube.push(b);
                }
            }
            boxes[label] = cube;
        } else {
            let parts: Vec<String> = s.split('-').map(|s| s.to_string()).collect();
            let label = hash(parts[0].clone());
            let mut cube = boxes[label].clone();
            for i in 0..cube.len() {
                if cube[i].label.eq(&parts[0]) {
                    cube.remove(i);
                    break;
                }
            }
            boxes[label] = cube;
        }
    }

    let mut sum = 0;
    for i in 0..boxes.len() {
        for j in 0..boxes[i].len() {
            sum += (i+1)*(j+1)*boxes[i][j].lens;
        }
    }
    
    println!("two star sol: {}", sum);
}
