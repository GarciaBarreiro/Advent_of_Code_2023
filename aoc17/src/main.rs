use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Tile {
    heat: u8,
    y: isize, 
    x: isize,
}

impl Tile {
    fn def(heat: u8) -> Self {
        Tile { heat, y: 0, x:0 }
    }
}

#[derive(Debug, Clone, Copy)]
struct Dir {
    dir: (isize,isize), // direction
    dist: u8,           // distance moved in that direction
}

fn main() {
    one_star(inp("./test"),0,4);
    one_star(inp("./test"),4,10);
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
            if let Ok(l) = line {
                // TODO: add to each tile its position
                map.push(l.chars().map(|n| Tile::def(n.to_string().parse::<u8>().unwrap())).collect());
            }
        }
    }
    
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            map[y][x].x = x as isize;
            map[y][x].y = y as isize;
        }
    }

    map
}

fn one_star(map: Vec<Vec<Tile>>, min_moves: u8, max_moves: u8) {
    let mut prev: HashMap<Tile,usize> = HashMap::new();
    let mut to_check: Vec<(Dir,Tile,usize)> = Vec::new();
    
    let end = (map.len() - 1, map[0].len() - 1);
    
    to_check.push((Dir {dir: (0,1), dist: 0}, map[0][0], 0));
    to_check.push((Dir {dir: (1,0), dist: 0}, map[0][0], 0));
    
    let mut res = usize::MAX;

    while let Some(curr) = to_check.pop() {
        let heat = prev.get(&curr.1).unwrap_or(&usize::MAX);
        //println!("curr = {:?},heat = {}", curr, heat);
        //std::thread::sleep(std::time::Duration::from_millis(10));
        if *heat > curr.2 {
            prev.insert(curr.1, curr.2);

            if curr.1.x as usize == end.1 && curr.1.y as usize == end.0 && curr.0.dist >= min_moves {
                if curr.2 < res {
                    res = curr.2;
                    break;
                }
                // break;
            } else if curr.1.x as usize == end.1 && curr.1.y as usize == end.0 { continue; }
            
            if curr.0.dist + 1 < max_moves {
                let next_y = curr.1.y + curr.0.dir.0;
                let next_x = curr.1.x + curr.0.dir.1;

                if next_y < 0 || next_y as usize >= map.len() || next_x < 0 || next_x as usize >= map[0].len() { continue; }
                
                to_check.push((Dir {dir: curr.0.dir, dist: curr.0.dist + 1},
                               map[next_y as usize][next_x as usize],
                               curr.2+map[next_y as usize][next_x as usize].heat as usize));
            }

            if curr.0.dist + 1 >= min_moves {
                let to_push = if curr.0.dir.0 == 0 { [(1,0),(-1,0)] }
                              else { [(0,1),(0,-1)] };
                for t in to_push {
                    let next_y = curr.1.y + t.0;
                    let next_x = curr.1.x + t.1;

                    if next_y < 0 || next_y as usize >= map.len() || next_x < 0 || next_x as usize >= map[0].len() { continue; }

                    to_check.push((Dir {dir: t, dist: 0},
                                   map[next_y as usize][next_x as usize],
                                   curr.2+map[next_y as usize][next_x as usize].heat as usize));
                }
            }
        }
    }

    println!("one star sol: {}", res);
}
