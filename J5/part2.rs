use std::fs::read_to_string;
use std::str::FromStr;

fn main(){
    let file_data = read_to_string("./inputs/input.txt").unwrap();
    let mut data_split = file_data.split("\n\n");

    let mut starts : Vec<u64> = Vec::new();
    let mut ends : Vec<u64> = Vec::new();

    for current_range in data_split.next().unwrap().split("\n") {
        let mut range_split = current_range.split("-");
        starts.push(u64::from_str(range_split.next().unwrap()).unwrap());
        ends.push(u64::from_str(range_split.next().unwrap()).unwrap());
    }

    starts.sort();
    let starts_len = starts.len();
    let mut start_idx = 0;

    ends.sort();
    let ends_len = starts.len();
    let mut end_idx = 0;

    let mut value_count = 0;
    while start_idx < starts_len && end_idx < ends_len {
        let current_start = starts[start_idx];
        start_idx += 1;
        let mut current_end = ends[end_idx];
        end_idx += 1;

        while start_idx < starts_len && starts[start_idx] <= current_end {
            start_idx += 1;
            if current_end <= ends[end_idx] {
                current_end = ends[end_idx];
                end_idx += 1;
            }
        }
        value_count += (current_end+1) - current_start;
    }
    println!("Possible ingredient count = {}", value_count);
}
