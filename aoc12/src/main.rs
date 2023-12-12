use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::sync::mpsc;
use std::thread;

#[derive(Debug, Clone)]
struct Spring {
    damaged: Vec<char>,
    springs: Vec<usize>,
}

fn main() {
    println!("one star sol: {}", arrangements(get_springs(0)));
    println!("two star sol: {}", arrangements_threaded(get_springs(4)));
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_springs(replicas: u8) -> Vec<Spring> {
    let mut spr: Vec<Spring> = Vec::new();
    if let Ok(lines) = read_lines("./input") {
        for line in lines {
            if let Ok(l) = line {
                let row: Vec<&str> = l.split_whitespace().collect();
                let mut springs: Vec<usize> = row[1].split(',')
                                                  .map(|n| n.parse::<usize>().unwrap())
                                                  .collect();
                let mut damaged: Vec<char> = row[0].chars().collect();
                let springs_og = springs.clone();
                let damaged_og = damaged.clone();
                for _ in 0..replicas {
                    springs.append(&mut springs_og.clone());
                    damaged.push('?');
                    damaged.append(&mut damaged_og.clone());
                }
                spr.push(Spring {damaged, springs});
            }
        }
    }
    spr
}

fn recursive(mut spr: Spring) -> usize {
    if spr.damaged.len() == 0 {
        if spr.springs.len() == 0 { return 1; }
        else { return 0; }
    }

    if spr.damaged[0] == '#' {
        if spr.springs.len() == 0 || spr.damaged.len() < spr.springs[0] {
            return 0;
        }

        if spr.damaged[..spr.springs[0]].contains(&'.') {
            return 0;
        }

        if spr.damaged.len() > spr.springs[0] && spr.damaged[spr.springs[0]] == '#' {
            return 0;
        }
        
        if spr.damaged.len() > spr.springs[0] && spr.damaged[spr.springs[0]] == '?' {
            return recursive(Spring { damaged: spr.damaged[spr.springs[0]+1..].to_vec(), springs: spr.springs[1..].to_vec() });
        }

        return recursive(Spring { damaged: spr.damaged[spr.springs[0]..].to_vec(), springs: spr.springs[1..].to_vec() });

    } else if spr.damaged[0] == '.' {
        spr.damaged.remove(0);
        return recursive(spr);
    } else /*if spr.damaged[0] == '?'*/ {
        let mut ret = 0;
        spr.damaged[0] = '#';
        ret += recursive(spr.clone());
        spr.damaged[0] = '.';
        ret += recursive(spr.clone());
        return ret;
    }
}

fn arrangements_threaded(spr: Vec<Spring>) -> usize {
    let mut sum = 0;

    let (tx, rx) = mpsc::channel();
    let data = spr.chunks(100);

    let mut joins: Vec<_> = Vec::new();
    for d in data {
        let owned_d = d.to_vec();
        let tx1 = tx.clone();
        joins.push(thread::spawn(move || {
            let mut ret = 0;
            for s in owned_d {
                println!("{:?}",s);
                ret += recursive(s.clone());
            }
            tx1.send(ret).unwrap();
        }));
    }

    for j in joins {
        j.join().unwrap();
    }

    for r in rx {
        println!("received = {:?}", r);
        sum += r;
        println!("sum = {:?}", sum);
    }

    sum
}

fn arrangements(spr: Vec<Spring>) -> usize {
    let mut sum = 0;

        for s in spr {
        sum += recursive(s);
    }

    sum
}
