use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Default, Debug, Clone)]
struct Pattern {
    rows: Vec<String>,
    h_index: Vec<isize>,    // index of all posible horizontal reflections
    v_index: Vec<isize>,    // same for vertical
}

fn main() {
    one_star(map());
    two_star(map(),1);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn map() -> Vec<Pattern> {
    let mut patterns: Vec<Pattern> = Vec::new();
    if let Ok(lines) = read_lines("./input") {
        let mut pat = Pattern::default();
        let mut prev_line = "".to_string();
        let mut i = 0;
        for line in lines {
            if let Ok(l) = line {
                if l.len() == 0 {
                    patterns.push(pat);
                    pat = Pattern::default();
                    prev_line = "".to_string();
                    i = 0;
                } else {
                    if l.eq(&prev_line) { pat.h_index.push(i); }
                    pat.rows.push(l.clone());
                    prev_line = l;
                    i += 1;
                }
            }
        }
    }
    patterns
}

fn one_star(pats: Vec<Pattern>) {
    let mut sol = 0;

    let mut temp: Vec<Pattern> = Vec::new();
    for p in pats {
        let mut found = false;
        for h in p.clone().h_index {
            let mut cont = true;
            let mut i: isize = 0;
            while cont {
                if (h + i) as usize == p.rows.len() || h - 1 - i < 0 {
                    cont = false;
                    found = true;
                } else {
                    if p.rows[(h+i) as usize].eq(&p.rows[(h-1-i) as usize]) {
                        i += 1;
                    } else { cont = false; }
                }
            }
            if found {
                sol += h * 100;
                break;
            }
        }
        if !found { temp.push(p); }
    }

    let mut vert: Vec<Pattern> = Vec::new();
    for mut t in temp {
        let mut prev_char = ' ';
        let mut i = 0;
        for c in t.rows[0].chars() {
            if c.eq(&prev_char) { t.v_index.push(i); } 
            i += 1;
            prev_char = c;
        }
        vert.push(t);
    }

    for p in vert {
        let mut found = false;
        for v in p.v_index {
            let mut cont = true;
            let mut i = 0;
            while cont {
                if (v + i) as usize == p.rows[0].len() || v - 1 - i < 0 {
                    cont = false;
                    found = true;
                } else {
                    let mut all_good = true;
                    for r in p.rows.clone() {
                        if !r.chars().collect::<Vec<_>>()[(v+i) as usize].eq(&r.chars().collect::<Vec<_>>()[(v-1-i) as usize]) {
                            all_good = false;
                        }
                        if !all_good { cont = false; break; }
                    }
                    i += 1;
                }
            }
            if found {
                sol += v;
                break;
            }
        }
    }

    println!("one star sol: {}", sol);
}

fn row_eq(r1: String, r2: String) -> usize {
    let mut found_sm = 0;
    let c1: Vec<char> = r1.chars().collect();
    let c2: Vec<char> = r2.chars().collect();
    for i in 0..r1.len() {
        if c1[i] != c2[i] {
            found_sm += 1;
        }   
    }
    found_sm
}

fn two_star(pats: Vec<Pattern>, limit: usize) {
    let mut sol = 0;

    let mut vert: Vec<Pattern> = Vec::new();
    for p in pats {
        let mut found = false;
        for h in 1..p.rows.len() as isize {
            let mut cont = true;
            let mut i: isize = 0;
            let mut found_sm = false;
            while cont {
                if ((h + i) as usize == p.rows.len() || h == i) && !found_sm {
                    break;
                }
                if ((h + i) as usize == p.rows.len() || h - 1 - i < 0) && found_sm {
                    cont = false;
                    found = true;
                } else {
                    let res = row_eq(p.rows[(h+i) as usize].clone(), p.rows[(h-1-i) as usize].clone());
                    if res == 0 {
                        i += 1;
                    } else if res == limit && !found_sm {
                        i += 1;
                        found_sm = true;
                    } else { cont = false; }
                }
            }
            if found {
                sol += h * 100;
                break;
            }
        }
        if !found { vert.push(p); }
    }

    for p in vert {
        let mut found = false;
        for v in 1..p.rows[0].len() as isize {
            let mut cont = true;
            let mut i = 0;
            let mut smudges = 0;
            while cont {
                if ((v + i) as usize == p.rows[0].len() || v == i) && smudges != limit {
                    break;
                }
                if ((v+i) as usize == p.rows[0].len() || v - 1 - i < 0) && smudges == limit {
                    cont = false;
                    found = true;
                } else {
                    let mut all_good = true;
                    for r in p.rows.clone() {
                        if !r.chars().collect::<Vec<_>>()[(v+i) as usize].eq(&r.chars().collect::<Vec<_>>()[(v-1-i) as usize]) {
                            if smudges == 0 { smudges += 1; }
                            else { all_good = false; }
                        }
                        if !all_good { cont = false; break; }
                    }
                    i += 1;
                }
            }
            if found {
                sol += v;
                break;
            }
        }
    }

    println!("two star sol: {}", sol);
}
