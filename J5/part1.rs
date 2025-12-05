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

    let mut final_starts : Vec<u64> = Vec::new();
    let mut final_ends : Vec<u64> = Vec::new();

    while start_idx < starts_len && end_idx < ends_len{
        final_starts.push(starts[start_idx]);
        start_idx += 1;
        let mut current_end = ends[end_idx];
        end_idx += 1;

        while start_idx < starts_len && starts[start_idx] <= current_end {
            start_idx += 1;
            if current_end <= ends[end_idx] {
                current_end = ends[end_idx];
                end_idx += 1;
            }
        } ;
        final_ends.push(current_end);
    }

    let mut final_count = 0;
    for current_query in data_split.next().unwrap().split("\n") {
        if current_query.len() == 0 { continue; }
        let query_value = u64::from_str(current_query).unwrap();

        let mut best :i32 = -1;

        let mut low : i32 = 0;
        let mut high  = final_starts.len() as i32 -1 ;
        while low <= high {
            let current_idx = (low + high) / 2;
            if final_starts[current_idx as usize] <= query_value {
                best = current_idx as i32;
                low = current_idx + 1;
            } else {
                high = current_idx - 1;
            }
        }
        final_count += (best != -1 && query_value <= final_ends[best as usize]) as i32;
    }
    println!("Valid ingredient count = {}", final_count);
}
