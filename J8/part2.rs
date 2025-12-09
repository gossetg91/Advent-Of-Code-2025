use std::collections::BinaryHeap;
use std::fs::read_to_string;
use std::str::FromStr;
use std::cmp::Ordering;

struct JunctBox {
    x : i64,
    y : i64,
    z : i64,
    node_group : i64
}

#[derive(PartialEq)]
struct JunctPair {
    j1 : usize,
    j2 : usize,
    dist : f64
}

impl Ord for JunctPair {
    fn cmp(&self, other: &Self) -> Ordering {
        other.dist
        .total_cmp(&self.dist)
        .then_with(
            || other.j1.cmp(&self.j1).then_with(
                || other.j2.cmp(&self.j2)
            )
        )
    }
}

impl PartialOrd for JunctPair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for JunctPair { }

fn get_dist(b1: &JunctBox, b2: &JunctBox) -> f64{
    let x_diff = b1.x - b2.x;
    let y_diff = b1.y - b2.y;
    let z_diff = b1.z - b2.z;

    return f64::sqrt((x_diff * x_diff + y_diff * y_diff + z_diff * z_diff) as f64 );
}

fn main() {
    let file_data = read_to_string("inputs/input.txt").unwrap();
    let mut junct_boxes = Vec::new();

    let mut current_group = 0;
    for current_line in file_data.split("\n") {
        if current_line.len() == 0 { continue; }
        let mut line_split = current_line.split(",");

        let new_junct_box = JunctBox {
            x : i64::from_str(line_split.next().unwrap()).unwrap(),
            y : i64::from_str(line_split.next().unwrap()).unwrap(),
            z : i64::from_str(line_split.next().unwrap()).unwrap(),
            node_group : current_group
        };

        junct_boxes.push(new_junct_box);
        current_group += 1;
    }

    let mut heap = BinaryHeap::new();
    for i in 0..junct_boxes.len() {
        for j in i+1..junct_boxes.len() {
            heap.push(JunctPair{j1:i, j2:j, dist:get_dist(&junct_boxes[i], &junct_boxes[j])});
        }
    }

    let mut last_prod = 0;
    while !heap.is_empty() {
        let current = heap.pop().unwrap();

        let target_group = junct_boxes[current.j1].node_group;
        let merged_group = junct_boxes[current.j2].node_group;
        if  target_group != merged_group {
            let mut other_group = false;

            for i in 0..junct_boxes.len() {
                if junct_boxes[i].node_group == merged_group{
                    junct_boxes[i].node_group = target_group;
                } else {
                    other_group = true;
                }
            }

            last_prod = junct_boxes[current.j1].x * junct_boxes[current.j2].x;

            if !other_group {
                break;
            }
        }
    }

    println!("Last edge : {}", last_prod);
}
