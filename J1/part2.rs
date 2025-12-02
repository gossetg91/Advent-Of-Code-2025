use std::fs::read_to_string;
use std::str::FromStr;

fn main() {
    let mut pos = 50;
    let mut count = 0;

    for current in read_to_string("./inputs/input.txt").unwrap().lines() {
        let dir = current.chars().nth(0).unwrap();
        let offset = i32::from_str(&current[1..]).unwrap();

        let mut offset_cleaned = offset;
        let to_left = dir == 'L';
        if pos != 0 {
            offset_cleaned = offset - if to_left { pos } else { 100 - pos };
            if offset_cleaned >= 0 { count += 1; }
        }
        count += offset_cleaned / 100;

        pos = (pos + offset * if to_left{ -1 } else { 1 }) % 100;
        if pos < 0 { pos = 100 + pos; }
    }
    println!("0 position pass: {}", count);
}
