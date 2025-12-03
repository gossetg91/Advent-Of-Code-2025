use std::fs::read_to_string;
use std::str::FromStr;

fn get_max(lower_bound : usize, upper_bound : usize, values : &Vec<i8>) -> (i64,usize){
    let mut max = values[lower_bound];
    let mut max_idx = lower_bound;
    for i in (lower_bound + 1)..upper_bound{
        let current = values[i];
        if current > max{
            max = current;
            max_idx = i;
            if current == 9 { break; }
        }
    }
    return (max as i64, max_idx)
}

fn main(){
    let data = read_to_string("./inputs/input.txt").unwrap();

    let mut sum : i64 = 0;
    for current in data.split("\n") {
        let values = current.chars().map(|char| i8::from_str(&char.to_string()).unwrap()).collect::<Vec<i8>>();
        if values.len() == 0{ continue; }

        let seq_length = 12;
        let mut current_mul : i64 = 100000000000;
        let mut current_start = 0;
        for i in 1..=seq_length {
            let (max, max_idx) = get_max(current_start, values.len()-(seq_length-i), &values);
            sum += max * current_mul;
            current_start = max_idx+1;
            current_mul /= 10;
        }
    }
    println!("Result: {}", sum);
}
