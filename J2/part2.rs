use std::fs::read_to_string;
use std::str::FromStr;

fn main() {
    let input_data = read_to_string("./inputs/input.txt").unwrap();
    let mut incorrect_sum = 0;

    for current in input_data.split(',') {
        let bounds = current.split('-').collect::<Vec<_>>();

        let begin = i128::from_str(bounds[0].trim()).unwrap();
        let end = i128::from_str(bounds[1].trim()).unwrap();

        for current_val in begin..=end {
            let value = current_val.to_string();
            let str_len = value.len();

            for span in (1..=str_len/2).rev() {
                if (str_len % span) != 0 { continue; }
                let pattern = &value[0..span];

                let mut valid = true;
                for current_begin in (span..str_len).step_by(span){
                    let to_check = &value[current_begin..current_begin+span];
                    if to_check != pattern {
                        valid = false;
                        break;
                    }
                }
                if valid {
                    incorrect_sum += current_val;
                    break;
                }
            }
        }
    }
    println!("Résultat : {}", incorrect_sum);
}
