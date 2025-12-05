use std::fs::read_to_string;

fn main() {
    let move_x = [-1,-1,-1, 0, 0, 1, 1, 1];
    let move_y = [-1, 0, 1,-1, 1,-1, 0, 1];

    let mut rows :i32 = 0;
    let cols;

    let mut map : Vec<char> = Vec::new();
    for current in read_to_string("./inputs/input.txt").unwrap().split("\n") {
        if current.len() == 0 {continue;}
        rows += 1;
        for current_char in current.chars() {
            map.push(current_char);
        }
    }
    cols = map.len() as i32 / rows;

    let mut accessible = 0;
    for i in 0..rows as i32 {
        for j in 0..cols {
            let current_index = (i * cols + j) as usize;
            if map[current_index] != '@' { continue; }

            let mut count = 0;
            for k in 0..8 {
                let pos_x = i + move_x[k];
                let pos_y = j + move_y[k];

                if pos_x >= 0 && pos_x < rows && pos_y >= 0 && pos_y < cols {
                    count += (map[(pos_x * cols + pos_y) as usize ] == '@') as i32;
                }
            }
            accessible += (count < 4) as i32;
        }
    }

    println!("Accessible : {}", accessible);
}
