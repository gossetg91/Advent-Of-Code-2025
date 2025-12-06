use std::fs::read_to_string;
use regex::Regex;

fn main() {
    let multi_space_sep = Regex::new(r"\s+").unwrap();
    let file_content = read_to_string("../inputs/input.txt").unwrap();

    let mut operand_seq = Vec::new();
    let mut ops = Vec::new();
    let mut results = Vec::new();

    let mut first_row = true;
    for current_line in file_content.split("\n") {
        let first = current_line.chars().nth(0).unwrap();

        if first == '+' || first == '*' {
            for current_op in multi_space_sep.split(current_line) {
                if current_op.len() == 0 { continue; }
                ops.push(current_op);
                results.push((current_op == "*") as i64)
            }
            break;
        }

        let mut current_idx = 0;
        for current_char in current_line.chars() {
            if first_row { operand_seq.push(Vec::new()); }
            operand_seq[current_idx].push(current_char);
            current_idx += 1;
        }
        first_row = false;
    }

    let mut op_idx = 0;
    for current_seq in operand_seq {
        let mut tmp_val = 0;

        let mut current_mult = 1;
        let mut full_skip = true;
        for current_char in current_seq.into_iter().rev() {
            if current_char == ' ' { continue; }
            full_skip = false;
            tmp_val += current_char.to_digit(10).unwrap() as i64 * current_mult;
            current_mult *= 10;
        }

        if full_skip {
            op_idx += 1;
            continue;
        }

        if ops[op_idx] == "+" {
            results[op_idx] += tmp_val;
        } else {
            results[op_idx] *= tmp_val;
        }
    }

    let mut final_result = 0;
    for current_res in results {
        final_result += current_res;
    }

    println!("Final result: {}", final_result);
}
