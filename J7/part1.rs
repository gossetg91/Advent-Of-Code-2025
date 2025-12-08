use std::fs::read_to_string;

fn main() {
    let file_data = read_to_string("./inputs/input.txt").unwrap();
    let mut data_split = file_data.split("\n");

    let first_line = data_split.next().unwrap();
    let start_pos = first_line.chars().position(|c| c == 'S').unwrap();
    let prob_len = first_line.len();

    let mut beam_status = Vec::new();
    for _i in 0..prob_len {
        beam_status.push(false);
    }
    beam_status[start_pos] = true;

    let mut indexing = Vec::new();
    let mut data = Vec::new();

    indexing.push(0);
    let mut current_idx = 0;
    for current in data_split {
        if current.len() == 0 { continue; }
        let old_idx = current_idx;
        let char_data = current.chars().collect::<Vec<char>>();
        for i in 0..prob_len {
            if char_data[i] == '^' {
                current_idx += 1;
                data.push(i);
            }
        }
        if current_idx != old_idx {
            indexing.push(current_idx);
        }
    }

    let mut split_count = 0;
    for i in 1..indexing.len() {
        for j in indexing[i-1]..indexing[i] {
            let current_pos = data[j];
            if beam_status[current_pos]{
                beam_status[current_pos] = false;
                if current_pos > 0 {
                    beam_status[current_pos-1] = true;
                }
                if current_pos < prob_len-1 {
                    beam_status[data[j]+1] = true;
                }
                split_count += 1;
            }
        }
    }
    println!("Split count : {}", split_count);
}
