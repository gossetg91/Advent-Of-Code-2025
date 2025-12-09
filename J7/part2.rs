use std::fs::read_to_string;
use std::collections::BTreeMap;


fn dfs_step(depth : usize, max_depth : usize, prob_len: usize, beam_pos: usize, indexing : &Vec<usize>, data: &Vec<usize>, mem: &mut BTreeMap<usize, BTreeMap<usize, i64>>) -> i64 {
    if !mem.contains_key(&depth) {
        mem.insert(depth, BTreeMap::new());
    } else if mem[&depth].contains_key(&beam_pos) {
        return mem[&depth][&beam_pos];
    }

    if depth == max_depth { return 1; }

    let mut no_hit = true;
    let mut path_count = 0;
    for i in indexing[depth-1]..indexing[depth] {
        let current_idx = data[i];
        if beam_pos == current_idx {
            no_hit = false;
            if current_idx > 0 {
                path_count += dfs_step(depth + 1, indexing.len(), prob_len, current_idx-1, indexing, data, mem);
            }
            if current_idx < prob_len{
                path_count += dfs_step(depth + 1, indexing.len(), prob_len, current_idx+1, indexing, data, mem);
            }
            break;
        }
    }

    if no_hit {
        path_count += dfs_step(depth + 1, indexing.len(), prob_len, beam_pos, indexing, data, mem);
    }

    if let Some(val) = mem.get_mut(&depth) { val.insert(beam_pos, path_count); };
    return path_count;
}

fn main() {
    let file_data = read_to_string("./inputs/input.txt").unwrap();
    let mut data_split = file_data.split("\n");

    let first_line = data_split.next().unwrap();
    let start_pos = first_line.chars().position(|c| c == 'S').unwrap();
    let prob_len = first_line.len();

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

    let mut mem : BTreeMap<usize, BTreeMap<usize, i64>> = BTreeMap::new();
    let path_count = dfs_step(1, indexing.len(), prob_len, start_pos, &indexing, &data, &mut mem);
    println!("Path count : {}", path_count);
}
