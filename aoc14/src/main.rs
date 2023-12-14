use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Rocks {
    None,
    Cube,
    Round,
}

fn main() {
    let (sol, _) = tilt(read_dish("./input"), 'n');
    println!("one star sol: {}", sol);
    let mut i: isize = 0;
    let mut final_dish: Vec<Vec<Vec<Rocks>>> = Vec::new();
    let mut dish = read_dish("./input");
    loop {
        let (_, dish_n) = tilt(dish, 'n');
        let (_, dish_w) = tilt(dish_n, 'w');
        let (_, dish_s) = tilt(dish_w, 's');
        let (_, dish_e) = tilt(dish_s, 'e');
        if final_dish.contains(&dish_e) {
            let repeat = final_dish.iter().position(|s| s.eq(&dish_e)).unwrap();
            let final_pos = (1000000000 - i) % (repeat as isize - i) + i - 1;
            println!("two star sol: {}", calc_weight(&final_dish[final_pos as usize - 14]));
            break;
        }
        i += 1;
        final_dish.push(dish_e.clone());
        dish = dish_e;
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[allow(dead_code)]
fn print_dish(dish: Vec<Vec<Rocks>>) {
    for d in dish {
        for r in d {
            if r == Rocks::None {
                print!(" ");
            } else if r == Rocks::Cube {
                print!("#");
            } else { print!("O"); }
        }
        println!("");
    }
}

fn read_dish(input: &str) -> Vec<Vec<Rocks>> {
    let mut dish: Vec<Vec<Rocks>> = Vec::new();
    if let Ok(lines) = read_lines(input) {
        for line in lines {
            if let Ok(l) = line {
                dish.push(l.chars().map(|c| if c == '.' { Rocks::None } else if c == '#' { Rocks::Cube } else { Rocks::Round }).collect());
            }
        }
    }

    dish
}

fn calc_weight(dish: &Vec<Vec<Rocks>>) -> usize {
    let mut sum = 0;
    for i in 0..dish.len() {
        for j in 0..dish[i].len() {
            if dish[i][j] == Rocks::Round {
                sum += dish.len() - i;
            }
        }
    }
    sum
}

fn tilt(mut dish: Vec<Vec<Rocks>>, dir: char) -> (usize, Vec<Vec<Rocks>>) {
    // print_dish(dish.clone());

    let n_range: Vec<usize> = (0..dish.len()).collect();
    let s_range: Vec<usize> = (0..dish.len()).rev().collect();
    let w_range: Vec<usize> = (0..dish[0].len()).collect();
    let e_range: Vec<usize> = (0..dish[0].len()).rev().collect();

    let vert_range = if dir == 's' { s_range } else { n_range };
    let hor_range = if dir == 'e' { e_range } else { w_range };

    //println!("{}, {:?}, {:?}", dir, vert_range, hor_range);

    for i in vert_range {
        for j in hor_range.clone() {
            if dish[i][j] == Rocks::Round {
                let mut steps = 0;
                if dir == 'n' || dir == 's' {
                    let k_dir: Vec<usize> = if dir == 's' { (i+1..dish.len()).collect() }
                                            else { (0..i).rev().collect() };
                    //println!("{:?}", k_dir);
                    for k in k_dir {
                        if dish[k][j] != Rocks::None {
                            break;
                        }
                        steps += 1;
                    }

                    dish[i][j] = Rocks::None;
                    if dir == 'n' { dish[i - steps][j] = Rocks::Round; }
                    else { dish[i + steps][j] = Rocks::Round; }
                } else if dir == 'w' || dir == 'e' {
                    let k_dir: Vec<usize> = if dir == 'e' { (j+1..dish[i].len()).collect() }
                                            else { (0..j).rev().collect() };
                    //println!("{:?}", k_dir);
                    for k in k_dir {
                        if dish[i][k] != Rocks::None {
                            break;
                        }
                        steps += 1;
                    }

                    dish[i][j] = Rocks::None;
                    if dir == 'w' { dish[i][j - steps] = Rocks::Round; }
                    else { dish[i][j + steps] = Rocks::Round; }
                }
            }
        }
    }

    /*
    let mut sum = 0;

    for i in 0..dish.len() {
        for j in 0..dish[i].len() {
            if dish[i][j] == Rocks::Round {
                sum += dish.len() - i;
            }
        }
    }
    */

    /*
    println!("");
    print_dish(dish.clone());
    */

    (calc_weight(&dish), dish)
}
