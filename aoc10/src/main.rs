use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    second_star(double_size(first_star()));
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Clone,Copy)]
struct Tiles {
    n: bool,        // has connection to the N
    s: bool,        // same four south
    w: bool,        // west
    e: bool,        // east
    start: bool,    // is it the start?
    outside: bool,  // is it outside?
    empty: bool,    // is it empty?
}

impl Default for Tiles {
    fn default() -> Self {
        Tiles { n: false, s: false, w: false, e: false,
                start: false, outside: false, empty: true }
    }
}

fn print_map(m: &Vec<Vec<Tiles>>) {
    for y in m {
        for x in y {
            if x.start {
                print!("S");
            } else if x.n && x.s {
                print!("│");
            } else if x.n && x.e {
                print!("└");
            } else if x.n && x.w {
                print!("┘");
            } else if x.s && x.e {
                print!("┌");
            } else if x.s && x.w {
                print!("┐");
            } else if x.e && x.w {
                print!("─");
            } else {
                if x.outside { print!("█"); }
                else { print!(" "); }
            }
        }
        println!("");
    }
}

fn first_star() -> Vec<Vec<Tiles>> {
    let mut map: Vec<Vec<Tiles>> = Vec::new();      // with all pipes, even those outside loop
    let mut start = [0,0];
    if let Ok(lines) = read_lines("./input") {
        for line in lines {
            if let Ok(l) = line {
                let mut v: Vec<Tiles> = Vec::new();
                for c in l.chars() {
                    match c {
                        '|' => v.push(Tiles {n: true, s: true, empty: false, ..Default::default()} ),
                        '-' => v.push(Tiles {w: true, e: true, empty: false, ..Default::default()} ),
                        'L' => v.push(Tiles {n: true, e: true, empty: false, ..Default::default()} ),
                        'J' => v.push(Tiles {n: true, w: true, empty: false, ..Default::default()} ),
                        '7' => v.push(Tiles {s: true, w: true, empty: false, ..Default::default()} ),
                        'F' => v.push(Tiles {s: true, e: true, empty: false, ..Default::default()} ),
                        'S' => {
                                v.push(Tiles {start: true, empty: false, ..Default::default()} );
                                start = [map.len(),v.len()-1];
                               },
                        _ => v.push(Tiles {..Default::default()} ),
                    }
                }
                map.push(v);
            }
        }
    }

    let mut graph: Vec<Vec<Tiles>> = vec![vec![Tiles {..Default::default()};map[0].len()];map.len()];
    graph[start[0]][start[1]] = Tiles {start: true, ..Default::default()};
    
    let mut dir = '?';
    // we know where start is
    // now we have to check at most all 4 squares around start, to now the next one to read
    if map[start[0] - 1][start[1]].s {
        dir = 'n';
    } else if map[start[0] + 1][start[1]].n {
        dir = 's';
    } else if map[start[0]][start[1] - 1].e {
        dir = 'w';
    } else if map[start[0]][start[1] + 1].w {
        dir = 'e';
    }

    let mut finished = false;
    let mut pos = start;
    let mut length = 0;
    while !finished {
        match dir {
            'n' => {
					pos[0] -= 1;
                    if map[pos[0]][pos[1]].w { dir = 'w'; }
                    else if map[pos[0]][pos[1]].e { dir = 'e'; }
                    // else direction is still north, no need to change (we can't turn 180 degrees)
				   },
            's' => {
					pos[0] += 1;
                    if map[pos[0]][pos[1]].w { dir = 'w'; }
                    else if map[pos[0]][pos[1]].e { dir = 'e'; }
				   },
            'w' => {
					pos[1] -= 1;
                    if map[pos[0]][pos[1]].n { dir = 'n'; }
                    else if map[pos[0]][pos[1]].s { dir = 's'; }
				   },
            'e' => {
					pos[1] += 1;
                    if map[pos[0]][pos[1]].n { dir = 'n'; }
                    else if map[pos[0]][pos[1]].s { dir = 's'; }
				   },
            _ => println!("what"),
        }
        graph[pos[0]][pos[1]] = map[pos[0]][pos[1]];
        if map[pos[0]][pos[1]].start { finished = true; }
        length += 1;
    }
    println!("one star sol: {}", length/2);

    graph
}

// if we double the size there's no need to check for squeezing,
// there'll be an open path to the outside
fn double_size(map: Vec<Vec<Tiles>>) -> Vec<Vec<Tiles>> {
    let mut double: Vec<Vec<Tiles>> = vec![vec![Tiles {..Default::default()};map[0].len()*2];map.len()*2];
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            double[y*2][x*2] = map[y][x];
            if map[y][x].start {
                if map[y+1][x].s {
                    double[y*2+1][x*2] = Tiles {n: true, s: true, empty: false, ..Default::default()};
                }
                if map[y][x+1].e {
                    double[y*2][x*2+1] = Tiles {w: true, e: true, empty: false, ..Default::default()};
                }
            } else {
                if map[y][x].s {
                    double[y*2+1][x*2] = Tiles {n: true, s: true, empty: false, ..Default::default()};
                }
                if map[y][x].e {
                    double[y*2][x*2+1] = Tiles {w: true, e: true, empty: false, ..Default::default()};
                }
            }
        }
    }
    double
}

// to delete all empty, outside tiles that were created when we doubled it
// we revert the map to its original size
fn halve_size(map: Vec<Vec<Tiles>>) -> Vec<Vec<Tiles>> {
    let mut half: Vec<Vec<Tiles>> = vec![vec![Tiles {..Default::default()};map[0].len()/2];map.len()/2];
    for y in (0..map.len()).step_by(2) {
        for x in (0..map[y].len()).step_by(2) {
            half[y/2][x/2] = map[y][x];
        }
    }
    half

}

fn second_star(mut map: Vec<Vec<Tiles>>) {
    let mut one_changed = true;
    while one_changed {
        one_changed = false;
        for y in 0..map.len() {
            for x in 0..map[y].len() {
                if map[y][x].empty && !map[y][x].outside {
                    if x == 0 || y == 0 || x == map[y].len() - 1 || y == map.len() - 1 {
                        map[y][x].outside = true;
                        one_changed = true;
                    } else if map[y-1][x].outside || map[y][x-1].outside {
                        map[y][x].outside = true;
                        one_changed = true;
                    }
                }
            }
        }
        for y in (0..map.len()).rev() {
            for x in (0..map[y].len()).rev() {
                if map[y][x].empty && !map[y][x].outside {
                    if x == 0 || y == 0 || x == map[y].len() - 1 || y == map.len() - 1 {
                        map[y][x].outside = true;
                        one_changed = true;
                    } else if map[y+1][x].outside || map[y][x+1].outside {
                        map[y][x].outside = true;
                        one_changed = true;
                    }
                }
            }
        }
    }
    let half = halve_size(map);
    let mut count = 0;
    for y in 0..half.len() {
        for x in 0..half[y].len() {
            if !half[y][x].empty || half[y][x].outside {
                count += 1;
            }
        }
    }
    print_map(&half);
    println!("two star sol = {}", half.len()*half[0].len() - count);
}
