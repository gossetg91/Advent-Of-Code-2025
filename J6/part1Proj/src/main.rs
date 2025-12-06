use std::fs::read_to_string;
use std::str::FromStr;
use regex::Regex;

fn main() {
    let multi_space_sep = Regex::new(r"\s+").unwrap();
    let file_content = read_to_string("../inputs/input.txt").unwrap();

    let mut operand_lines = Vec::new();
    let mut ops = Vec::new();
    let mut results = Vec::new();

    for current_line in file_content.split("\n") {
        let first = current_line.chars().nth(0).unwrap();

        if first == '+' ||  first == '*' {
            for current_op in multi_space_sep.split(current_line) {
                if current_op.len() == 0 { continue; }
                ops.push(current_op);
                results.push((current_op == "*") as i64)
            }
            break;
        }

        operand_lines.push(current_line);
    }

    for current_line in operand_lines {
        let mut current_idx = 0;
        for current_operand in multi_space_sep.split(current_line) {
            if current_operand.len() == 0 { continue; }
            let op_converted = i64::from_str(current_operand).unwrap();

            if ops[current_idx] == "+" {
                results[current_idx] += op_converted;
            } else {
                results[current_idx] *= op_converted;
            }
            current_idx += 1;
        }
    }

    let mut final_result = 0;
    for current_res in results {
        final_result += current_res;
    }
    println!("Final result: {}", final_result);
}
