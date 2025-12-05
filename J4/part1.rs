use std::fs::read_to_string;

fn main() {
    let mut map : Vec<bool> = Vec::new();
    let inner_size;
    let mut rows = 0;

    for current in read_to_string("./inputs/input.txt").unwrap().split("\n") {
        if current.len() == 0 {continue;}
        rows += 1;
        for current_char in current.chars() {
            map.push(current_char == '@');
        }
    }
    inner_size = (map.len() / rows) as i32;

    let mut accessible = 0;
    for i in 0..rows as i32 {
        for j in 0..inner_size {
            if !map[(i * inner_size + j) as usize] { continue; }

            let mut count = 0;
            'outer: for ii in if i == 0 {i} else {i - 1}..=if i < rows as i32 - 1 {i + 1} else {i} {
                let on_verif_line = ii == i;
                for jj in (if j == 0 { if on_verif_line {j + 1} else {j}} else {j - 1}..=if j < inner_size - 1 {j + 1} else {j}).step_by(if on_verif_line {2} else {1}) {
                    if map[(ii * inner_size + jj) as usize] {
                        count += 1;
                        if count >= 4 { break 'outer; }
                    }
                }
            }
            if count < 4 {
                accessible += 1;
            }
        }
    }

    println!("Accessible : {}", accessible);
}
