use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

fn _read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    first_star();
    second_star();
}

fn first_star() {
    let re = Regex::new(r"Game (\d+): (.+)").unwrap();
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;
    let mut sum = 0;
    let mut total = 0;
    if let Ok(lines) = _read_lines("./input") {
        for line in lines {
            if let Ok(res) = line {
                for (_, [id, results]) in re.captures_iter(&res).map(|c| c.extract()) {
                    total += id.to_string().parse::<i32>().unwrap();
                    let mut found = false;
                    let games = results.split("; ");
                    for game in games {
                        let sets = game.split(", ");
                        for set in sets {
                            let (num, color) = set.split_once(" ").unwrap();
                            if "red".eq(color) {
                                if num.to_string().parse::<i32>().unwrap() > max_red && !found {
                                    sum += id.to_string().parse::<i32>().unwrap();
                                    found = true;
                                }
                            } else if "green".eq(color) {
                                if num.to_string().parse::<i32>().unwrap() > max_green && !found {
                                    sum += id.to_string().parse::<i32>().unwrap();
                                    found = true;
                                }
                            } else {
                                if num.to_string().parse::<i32>().unwrap() > max_blue && !found {
                                    sum += id.to_string().parse::<i32>().unwrap();
                                    found = true;
                                }
                            }
                        }
                    }
                }
            }
        }
        println!("one star sum = {}", total - sum);
    }
}

fn second_star() {
    let re = Regex::new(r"Game (\d+): (.+)").unwrap();
    let mut power = 0;
    if let Ok(lines) = _read_lines("./input") {
        for line in lines {
            if let Ok(res) = line {
                for (_, [_, results]) in re.captures_iter(&res).map(|c| c.extract()) {
                    let mut min_red = 0;
                    let mut min_green = 0;
                    let mut min_blue = 0;
                    let games = results.split("; ");
                    for game in games {
                        let sets = game.split(", ");
                        for set in sets {
                            let (num, color) = set.split_once(" ").unwrap();
                            if "red".eq(color) {
                                if num.to_string().parse::<i32>().unwrap() > min_red {
                                    min_red = num.to_string().parse::<i32>().unwrap();
                                }
                            } else if "green".eq(color) {
                                if num.to_string().parse::<i32>().unwrap() > min_green {
                                    min_green = num.to_string().parse::<i32>().unwrap();
                                }
                            } else {
                                if num.to_string().parse::<i32>().unwrap() > min_blue {
                                    min_blue = num.to_string().parse::<i32>().unwrap();
                                }
                            }
                        }
                    }
                    power += min_red * min_green * min_blue;
                }
            }
        }
        println!("two star sum = {}", power);
    }
}
