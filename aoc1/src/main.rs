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
    let mut total = 0;
    if let Ok(lines) = read_lines("./input") {
        for line in lines {
            if let Ok(cal) = line {
                let mut found_first = false;
                let mut first = 0;
                let mut second = 10;
                for c in cal.chars() {
                    if c.is_digit(10) {
                        if !found_first {
                            first = c.to_digit(10).unwrap();
                            found_first = true;
                        } else {
                            second = c.to_digit(10).unwrap();
                        }
                    }
                }
                if second < 10 { total += first*10 + second; }
                else { total += first*10 + first; }
            }
        }
    }
    println!("First Star Total = {}", total);
}

fn second_star() {
    let mut total = 0;
    if let Ok(lines) = read_lines("./input") {
        for line in lines {
            if let Ok(cal) = line {
                let mut found_first = false;
                let mut first = 0;
                let mut second = 10;
                let mut state = 0;
                for c in cal.chars() {
                    match c {
                        'o' => { match state {
                                0 => state = 1,
                                4 =>  { if !found_first { first = 2; found_first = true; } else { second = 2; } state = 1; }, 
                                8 => state = 9,
                                _ => state = 1,
                            }
                        },
                        't' => { match state {
                                0 => state = 3,
                                21 => { if !found_first { first = 8; found_first = true; } else { second = 8; } state = 3; },
                                _ => state = 3,
                            }
                        },
                        'f' => state = 8,
                        's' => state = 13,
                        'e' => { match state {
                                0 => state = 18,
                                2 => { if !found_first { first = 1; found_first = true; } else { second = 1; } state = 18; },
                                6 => state = 7,
                                7 => { if !found_first { first = 3; found_first = true; } else { second = 3; } state = 18; },
                                12 => { if !found_first { first = 5; found_first = true; } else { second = 5; } state = 18; },
                                13 => state = 15,
                                16 => state = 17,
                                24 => { if !found_first { first = 9; found_first = true; } else { second = 9; } state = 18; },
                                _ => state = 18,
                            }
                        },
                        'n' => { match state {
                                0 => state = 22,
                                1 => state = 2,
                                9 => state = 2,
                                17 =>{ if !found_first { first = 7; found_first = true; } else { second = 7; } state = 22; }, 
                                23 => state = 24,
                                _ => state = 22,
                            }
                        },
                        'w' => { match state {
                                3 => state = 4,
                                _ => state = 0,
                            }
                        },
                        'h' => { match state {
                                3 => state = 5,
                                20 => state = 21,
                                _ => state = 0,
                            }
                        },
                        'r' => { match state {
                                5 => state = 6,
                                10 => { if !found_first { first = 4; found_first = true; } else { second = 4; } state = 0; },
                                _ => state = 0,
                            }
                        },
                        'u' => { match state {
                                9 => state = 10,
                                _ => state = 0,
                            }
                        },
                        'i' => { match state {
                                2 => state = 23,
                                7 => state = 19,
                                8 => state = 11,
                                13 => state = 14,
                                15 => state = 19,
                                17 => state = 19,
                                18 => state = 19,
                                22 => state = 23,
                                24 => state = 23,
                                _ => state = 0,
                            }
                        },
                        'v' => { match state {
                                11 => state = 12,
                                15 => state = 16,
                                _ => state = 0,
                            }
                        },
                        'x' => { match state {
                                14 => { if !found_first { first = 6; found_first = true; } else { second = 6; } state = 0; },
                                _ => state = 0,
                            }
                        },
                        'g' => { match state {
                                19 => state = 20,
                                _ => state = 0,
                            }
                        },
                        '1'..='9' => {if !found_first {
                                first = c.to_digit(10).unwrap();
                                found_first = true;
                            } else { second = c.to_digit(10).unwrap(); }
                            state = 0;
                        },
                        _ => state = 0,
                    }
                }
                if second < 10 { total += first*10 + second; }
                else { total += first*10 + first; }
            }
        }
    }
    println!("Second Star Total = {}", total);
}
