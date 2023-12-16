use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Default, Clone, Copy, PartialEq)]
enum Elem {
    #[default]
    None,
    R_mirror,
    L_mirror,
    V_splitter,
    H_splitter,
}

#[derive(Clone, Copy)]
struct Tile {
    energized: bool,
    content: Elem,
}

impl Default for Tile {
    fn default() -> Self {
        Tile { energized: false, content: Elem::default() }
    }
}

fn main() {
    println!("one star sol: {}",one_star(inp("./input"),(0,0,'r')));

    let map = inp("./input");
    let mut max = 0;
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if (i == 0 || i == map.len() - 1) || (j == 0 || j == map[i].len()-1) {
                let dir = 
                    if i == 0 { 'd' }
                    else if i == map.len() - 1 { 'u' }
                    else if j == 0 { 'r' }
                    else { 'l' };
                let sol = one_star(map.clone(), (i as isize, j as isize, dir));
                if sol > max { max = sol; }
            }
        }
    }
    println!("two star sol: {}", max);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn inp(input: &str) -> Vec<Vec<Tile>> {
    let mut map: Vec<Vec<Tile>> = Vec::new();
    if let Ok(lines) = read_lines(input) {
        for line in lines {
            let mut row: Vec<Tile> = Vec::new();
            if let Ok(l) = line {
                for c in l.chars() {
                    match c {
                        '/' => row.push(Tile { energized: false, content: Elem::R_mirror }),
                        '\\' => row.push(Tile { energized: false, content: Elem::L_mirror }),
                        '|' => row.push(Tile { energized: false, content: Elem::V_splitter }),
                        '-' => row.push(Tile { energized: false, content: Elem::H_splitter }),
                        _ => row.push(Tile::default()),
                    }
                }
            }
            map.push(row);
        }
    }
    map
}

fn one_star(mut map: Vec<Vec<Tile>>, start_pos: (isize,isize,char)) -> usize {
    let mut states: Vec<(isize, isize, char)> = Vec::new();     // current states where light is
    let mut visited: Vec<(isize, isize, char)> = Vec::new();    // previous states (so we don't end in a loop)

    let first_tile = map[start_pos.0 as usize][start_pos.1 as usize];

    // very ugly
    if first_tile.content == Elem::R_mirror {
        match start_pos.2 {
            'u' => states.push((start_pos.0,start_pos.1,'r')),
            'd' => states.push((start_pos.0,start_pos.1,'l')),
            'r' => states.push((start_pos.0,start_pos.1,'u')),
            _ => states.push((start_pos.0,start_pos.1,'d')),
        }
    } else if first_tile.content == Elem::L_mirror {
        match start_pos.2 {
            'u' => states.push((start_pos.0,start_pos.1,'l')),
            'd' => states.push((start_pos.0,start_pos.1,'r')),
            'r' => states.push((start_pos.0,start_pos.1,'d')),
            _ => states.push((start_pos.0,start_pos.1,'u')),
        }
    } else if first_tile.content == Elem::V_splitter {
        match start_pos.2 {
            'r' => {
                states.push((start_pos.0,start_pos.1,'u'));
                states.push((start_pos.0,start_pos.1,'d'));
            },
            'l' => {
                states.push((start_pos.0,start_pos.1,'u'));
                states.push((start_pos.0,start_pos.1,'d'));
            },
            _ => states.push(start_pos),
        }
    } else if first_tile.content == Elem::H_splitter {
        match start_pos.2 {
            'u' => {
                states.push((start_pos.0,start_pos.1,'r'));
                states.push((start_pos.0,start_pos.1,'l'));
            },
            'd' => {
                states.push((start_pos.0,start_pos.1,'r'));
                states.push((start_pos.0,start_pos.1,'l'));
            },
            _ => states.push(start_pos),
        }
    } else {
        states.push(start_pos);
    }

    for st in states.clone() {
        visited.push(st);
    }

    while !states.is_empty() {
        let curr_state = states.pop().unwrap();

        map[curr_state.0 as usize][curr_state.1 as usize].energized = true;
        
        let next_pos = 
            if curr_state.2 == 'u' { (curr_state.0-1,curr_state.1) }
            else if curr_state.2 == 'd' { (curr_state.0+1,curr_state.1) }
            else if curr_state.2 == 'r' { (curr_state.0,curr_state.1+1) }
            else { (curr_state.0,curr_state.1-1) };
        
        if next_pos.0 >= 0 && next_pos.0 < map.len() as isize
            && next_pos.1 >= 0 && next_pos.1 < map[0].len() as isize {

            let next_tile = map[next_pos.0 as usize][next_pos.1 as usize];
            if next_tile.content == Elem::R_mirror {
                match curr_state.2 {
                    'u' => states.push((next_pos.0,next_pos.1,'r')),
                    'd' => states.push((next_pos.0,next_pos.1,'l')),
                    'r' => states.push((next_pos.0,next_pos.1,'u')),
                    _ => states.push((next_pos.0,next_pos.1,'d')),
                }
            } else if next_tile.content == Elem::L_mirror {
                match curr_state.2 {
                    'u' => states.push((next_pos.0,next_pos.1,'l')),
                    'd' => states.push((next_pos.0,next_pos.1,'r')),
                    'r' => states.push((next_pos.0,next_pos.1,'d')),
                    _ => states.push((next_pos.0,next_pos.1,'u')),
                }
            } else if next_tile.content == Elem::V_splitter {
                match curr_state.2 {
                    'r' => {
                        states.push((next_pos.0,next_pos.1,'u'));
                        states.push((next_pos.0,next_pos.1,'d'));
                    },
                    'l' => {
                        states.push((next_pos.0,next_pos.1,'u'));
                        states.push((next_pos.0,next_pos.1,'d'));
                    },
                    _ => states.push((next_pos.0,next_pos.1,curr_state.2)),
                }
            } else if next_tile.content == Elem::H_splitter {
                match curr_state.2 {
                    'u' => {
                        states.push((next_pos.0,next_pos.1,'r'));
                        states.push((next_pos.0,next_pos.1,'l'));
                    },
                    'd' => {
                        states.push((next_pos.0,next_pos.1,'r'));
                        states.push((next_pos.0,next_pos.1,'l'));
                    },
                    _ => states.push((next_pos.0,next_pos.1,curr_state.2)),
                }
            } else {
                states.push((next_pos.0,next_pos.1,curr_state.2));
            }
        }

        if !states.is_empty() {
            let last_state = states.pop().unwrap();
            if !visited.contains(&last_state) {
                states.push(last_state);
                visited.push(last_state);
            }
        }
    }

    let mut num_energized = 0;
    
    for r in map {
        for t in r {
            if t.energized {
                num_energized += 1;
            }
        }
    }

    num_energized
}
