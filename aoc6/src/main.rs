use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

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
    if let Ok(lines) = read_lines("./input") {
        let mut time_found = false;
        let mut time: Vec<u32> = Vec::new();
        let mut dist: Vec<u32> = Vec::new();
        for line in lines {
            if let Ok(data) = line {
                let split = data.split_whitespace();
                let mut i = 0;
                if !time_found {
                    for parts in split {
                        if i != 0 {
                            time.push(parts.to_string().parse::<u32>().unwrap());
                        }
                        i+=1;
                        time_found = true;
                    }
                } else {
                    for parts in split {
                        if i != 0 {
                            dist.push(parts.to_owned().parse::<u32>().unwrap());
                        }
                        i+=1;
                    }
                }
            }
        }
        let mut dist_pos = 0;
        let mut num_ways = 1;
        for t in time {
            let mut times_beat = 0;
            for i in 0..t+1 {
                let dist_travel = (t - i) * i;
                if dist_travel > dist[dist_pos] { times_beat += 1; }
            }
            dist_pos += 1;
            num_ways *= times_beat;
        }
        println!("one star sol = {}", num_ways);
    }
}

fn second_star() {
    if let Ok(lines) = read_lines("./input") {
        let mut time_found = false;
        let mut time_str = String::new();
        let mut dist_str = String::new();
        for line in lines {
            if let Ok(data) = line {
                let split = data.split_whitespace();
                let mut i = 0;
                if !time_found {
                    for parts in split {
                        if i != 0 {
                            time_str.push_str(parts);
                        }
                        i+=1;
                        time_found = true;
                    }
                } else {
                    for parts in split {
                        if i != 0 {
                            dist_str.push_str(parts);
                        }
                        i+=1;
                    }
                }
            }
        }
        let time = time_str.to_string().parse::<u64>().unwrap();
        let dist = dist_str.to_string().parse::<u64>().unwrap();
        let mut num_ways = 0;
        for i in 0..time+1 {
            let dist_travel = (time - i) * i;
            if dist_travel > dist { num_ways += 1; }
        }
        println!("two star sol = {}", num_ways);
    }
}
