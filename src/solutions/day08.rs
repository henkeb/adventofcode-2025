use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

pub fn puzzle_2(input: &str) -> String {
    let junction_boxes = handle_input(input);
    let mut circuits = vec![0; junction_boxes.len()];
    let mut id_circuit = 0;
    let boxes = get_closest_circuits(&junction_boxes);

    for boxi in boxes.iter() {
        let i = boxi.a;
        let j = boxi.b;
        match (circuits[i], circuits[j]) {
            (0, 0) => {
                id_circuit += 1;
                circuits[i] = id_circuit;
                circuits[j] = circuits[i];
            }
            (_, 0) => {
                circuits[j] = circuits[i];
            }
            (0, _) => {
                circuits[i] = circuits[j];
            }
            (_, _) => {
                let i_circuit = circuits[i];
                let j_circuit = circuits[j];

                for circuit in circuits.iter_mut() {
                    if *circuit == j_circuit {
                        *circuit = i_circuit;
                    }
                }
            }
        }
        if circuits.iter().all(|&elem| elem == circuits[0]) {
            return (junction_boxes[boxi.a].0 * junction_boxes[boxi.b].0).to_string();
        }
    }
    "".to_string()
}

pub fn puzzle_1(input: &str) -> String {
    let junction_boxes = handle_input(input);
    let mut circuits = vec![0; junction_boxes.len()];
    let mut id_circuit = 0;
    let sorted_boxes = get_closest_circuits(&junction_boxes);
    let mut boxes = Vec::new();
    let mut n = 0;
    if junction_boxes.len() == 20 {
        n = 10;
    } else {
        n = 1000;
    }
    for i in 0..n {
        boxes.push(&sorted_boxes[i]);
    }
    for boxi in boxes {
        let i = boxi.a;
        let j = boxi.b;
        match (circuits[i], circuits[j]) {
            (0, 0) => {
                id_circuit += 1;
                circuits[i] = id_circuit;
                circuits[j] = circuits[i];
            }
            (_, 0) => {
                circuits[j] = circuits[i];
            }
            (0, _) => {
                circuits[i] = circuits[j];
            }
            (_, _) => {
                let i_circuit = circuits[i];
                let j_circuit = circuits[j];

                for circuit in circuits.iter_mut() {
                    if *circuit == j_circuit {
                        *circuit = i_circuit;
                    }
                }
            }
        }
    }

    let mut h_map: HashMap<i128, i128> = HashMap::new();

    let mut max_values: [i128; 3] = [0, 0, 0];

    for circuit in circuits {
        if circuit == 0 {
            continue;
        }
        h_map
            .entry(circuit)
            .and_modify(|num| *num += 1)
            .or_insert(1);
    }

    for (_, &v) in h_map.iter() {
        if v > max_values[0] {
            max_values[2] = max_values[1];
            max_values[1] = max_values[0];
            max_values[0] = v;
        } else if v > max_values[1] {
            max_values[2] = max_values[1];
            max_values[1] = v;
        } else if v > max_values[2] {
            max_values[2] = v;
        }
    }
    max_values.iter().product::<i128>().to_string()
}

fn get_closest_circuits(junction_boxes: &Vec<(i128, i128, i128)>) -> Vec<CircuitsDistance> {
    let mut heap = BinaryHeap::new();

    for (i, &(x, y, z)) in junction_boxes.iter().enumerate() {
        for (j, &(x_2, y_2, z_2)) in junction_boxes.iter().enumerate().skip(i) {
            if x == x_2 && y == y_2 && z == z_2 {
                continue;
            }
            let s_distance = ((x - x_2).pow(2) + (y - y_2).pow(2) + (z - z_2).pow(2)).isqrt();
            heap.push(Reverse(CircuitsDistance {
                distance: s_distance,
                a: i,
                b: j,
            }));
        }
    }

    let mut output = Vec::new();
    while let Some(Reverse(val)) = heap.pop() {
        output.push(val);
    }
    output
}

#[derive(Eq, PartialEq)]
struct CircuitsDistance {
    distance: i128,
    a: usize,
    b: usize,
}

impl Ord for CircuitsDistance {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance.cmp(&other.distance)
    }
}

impl PartialOrd for CircuitsDistance {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

fn handle_input(input: &str) -> Vec<(i128, i128, i128)> {
    input
        .lines()
        .map(|line| line.split(",").collect::<Vec<&str>>())
        .map(|nums| {
            (
                nums[0].parse().unwrap(),
                nums[1].parse().unwrap(),
                nums[2].parse().unwrap(),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    #[test]
    fn test_1() {
        assert_eq!(puzzle_1(&INPUT), "40");
    }

    #[test]
    fn test_2() {
        assert_eq!(puzzle_2(&INPUT), "25272");
    }
}
