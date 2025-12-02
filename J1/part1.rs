use std::fs::read_to_string;
use std::str::FromStr;

fn main(){
    let mut pos = 50;
    let mut count = 0;
    for current in read_to_string("./inputs/input.txt").unwrap().lines() {
        let dir = current.chars().nth(0).unwrap();
        let offset = i32::from_str(&current[1..]).unwrap() * (if dir=='L' {-1} else {1});

        pos = (pos + offset) % 100;
        if pos == 0 { count += 1; }
        else if pos < 0 { pos = 100 + pos; }
    }
    println!("0 position count : {}", count);
}
