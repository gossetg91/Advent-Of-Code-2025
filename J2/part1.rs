use std::fs::read_to_string;
use std::str::FromStr;

fn get_bound_above(value: i64) -> i64 {
    let mut to_return = 1;
    while value >= to_return { to_return *= 10; }

    return to_return;
}

fn get_bound_below(value: i64) -> i64 {
    let mut to_return = 1;
    while value > to_return*10 { to_return *= 10; }

    return to_return - 1;
}

fn get_complete_id(value: i64, digit_count : usize) -> i64 {
    let mut to_return = value;
    for _i in 0..digit_count { to_return *= 10; }

    return to_return + value;
}

fn main() {
    let input_data = read_to_string("./inputs/input.txt").unwrap();
    let mut incorrect_sum = 0;

    for current in input_data.split(',') {
        let bounds = current.split('-').collect::<Vec<_>>();

        let mut begin : String = bounds[0].trim().to_string();
        let mut end : String = bounds[1].trim().to_string();
        if begin.len() == end.len() && begin.len() % 2 != 0 { continue; }

        if begin.len() % 2 != 0 {
            let begin_val = i64::from_str(&begin).unwrap();
            begin = get_bound_above(begin_val).to_string();
        }

        if end.len() % 2 != 0 {
            let end_val = i64::from_str(&end).unwrap();
            end = get_bound_below(end_val).to_string();
        }

        let b_header_size = begin.len()/2;
        let mut begin_header = i64::from_str(&begin[..b_header_size]).unwrap();
        let begin_footer = i64::from_str(&begin[b_header_size..]).unwrap();

        let e_header_size = end.len()/2;
        let mut end_header = i64::from_str(&end[..e_header_size]).unwrap();
        let end_footer = i64::from_str(&end[e_header_size..]).unwrap();

        if begin_header == end_header {
            if begin_header >= begin_footer && end_header <= end_footer {
               incorrect_sum += get_complete_id(begin_header, b_header_size);
            }
            continue;
        }

        if begin_header >= begin_footer {
            incorrect_sum += get_complete_id(begin_header, b_header_size);
        }
        begin_header += 1;

        if end_header <= end_footer {
            incorrect_sum += get_complete_id(end_header, e_header_size);
        }
        end_header -= 1;

        if end_header < begin_header { continue; }

        for current in begin_header..=end_header {
            incorrect_sum += get_complete_id(current, current.to_string().len());
        }
    }
    println!("Résultat : {}", incorrect_sum);
}
