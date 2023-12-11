use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Clone, Copy)]
struct Galaxy {
    x: u64,
    y: u64,
}

fn main() {
    println!("one star sol: {}", distances(1));
    println!("two star sol: {}", distances(999999));
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn distances(expansion: u64) -> u64 {
    let mut space: Vec<Galaxy> = Vec::new();
    if let Ok(lines) = read_lines("./input") {
        let mut y = 0;
        for line in lines {
            if let Ok(l) = line {
                let mut x = 0;
                let mut found_galaxy = false;
                for c in l.chars() {
                    if c == '#' {
                        space.push(Galaxy {x, y});
                        found_galaxy = true;
                    }
                    x += 1;
                }
                y += 1;
                if !found_galaxy { y += expansion; }    // we expand rows now
            }
        }
    }

    let mut max_x = 0;
    let mut found_x: Vec<u64> = Vec::new();
    for g in &space {
        if !found_x.contains(&g.x) {
            found_x.push(g.x);
        }
        if g.x > max_x { max_x = g.x; }
    }
    let mut final_pos = 0;
    let mut expanded: Vec<Galaxy> = Vec::new(); // i can't find how to mutate the elements of the other vector
    for i in 0..=max_x {
        if !found_x.contains(&i) { final_pos += expansion; }
        else {
            for g in &space {
                if g.x == i {
                    expanded.push(Galaxy { x: final_pos, y: g.y });
                }
            }
        }
        final_pos += 1;
    }

    let mut tot_steps = 0;
    for i in 0..expanded.len() {
        for j in i+1..expanded.len() {
            tot_steps += (expanded[i].x).abs_diff(expanded[j].x) + (expanded[i].y).abs_diff(expanded[j].y);
        }
    }
    tot_steps
}
