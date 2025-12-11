use std::collections::{HashSet, VecDeque};

pub fn puzzle_1(input: &str) -> String {
    let machines = handle_input(input);
    let mut output = 0;
    for machine in machines {
        let mut queue = VecDeque::new();
        let mut memo = HashSet::new();
        queue.push_back((vec![false; machine.0.len()], 0));
        while let Some((curr_indicator_lights, button_presses)) = queue.pop_front() {
            if memo.contains(&(curr_indicator_lights)) {
                continue;
            }
            if curr_indicator_lights == machine.0 {
                output += button_presses;
                break;
            }
            memo.insert(curr_indicator_lights.clone());
            for button_list in &machine.1 {
                let mut next_indicator_lights = curr_indicator_lights.clone();
                for button in button_list {
                    next_indicator_lights[*button] = !next_indicator_lights[*button];
                }
                queue.push_back((next_indicator_lights, button_presses + 1));
            }
        }
    }
    output.to_string()
}

pub fn puzzle_2(input: &str) -> String {
    let machines = handle_input(input);
    let mut output = 0;
    for (_, buttons, joltages) in machines.iter() {
        let opt = z3::Optimize::new();

        let vars: Vec<_> = (0..buttons.len())
            .map(|i| z3::ast::Int::new_const(i as u32))
            .collect();

        for var in &vars {
            opt.assert(&var.ge(&z3::ast::Int::from_i64(0)));
        }

        for (i, &joltage) in joltages.iter().enumerate() {
            let joltage = joltage as u64;
            let mut button_mapping_to_joltage = Vec::new();
            for (j, button_mapping) in buttons.iter().enumerate() {
                for &button in button_mapping.iter() {
                    if button == i {
                        button_mapping_to_joltage.push(vars[j].clone());
                    }
                }
            }

            let sum = button_mapping_to_joltage
                .into_iter()
                .reduce(|a, b| a + b)
                .unwrap();
            let target_joltage = z3::ast::Int::from_u64(joltage);
            opt.assert(&sum.eq(&target_joltage));
        }

        let button_presses: z3::ast::Int =
            vars.iter().map(|v| v.clone()).reduce(|a, b| a + b).unwrap();
        opt.minimize(&button_presses);

        if opt.check(&[]) == z3::SatResult::Sat {
            output += opt
                .get_model()
                .unwrap()
                .eval(&button_presses, false)
                .unwrap()
                .as_u64()
                .unwrap();
        }
    }
    output.to_string()
}

fn handle_input(input: &str) -> Vec<(Vec<bool>, Vec<Vec<usize>>, Vec<usize>)> {
    input
        .lines()
        .map(|line| {
            let splits: Vec<&str> = line.split(" ").collect();
            let indicator_lights: Vec<bool> = splits[0][1..splits[0].len() - 1]
                .chars()
                .map(|ch| ch == '#')
                .collect();
            let buttons: Vec<Vec<usize>> = splits[1..splits.len() - 1]
                .iter()
                .map(|&button| {
                    button[1..button.len() - 1]
                        .split(",")
                        .map(|button| button.parse::<usize>().unwrap())
                        .collect()
                })
                .collect();
            let joltage: Vec<usize> = splits
                .last()
                .map(|&joltage| {
                    joltage[1..joltage.len() - 1]
                        .split(",")
                        .map(|joltage| joltage.parse::<usize>().unwrap())
                })
                .unwrap()
                .collect();
            (indicator_lights, buttons, joltage)
        })
        .collect::<Vec<(Vec<bool>, Vec<Vec<usize>>, Vec<usize>)>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    #[test]
    fn test_1() {
        assert_eq!(puzzle_1(&INPUT), "7");
    }

    #[test]
    fn test_2() {
        assert_eq!(puzzle_2(&INPUT), "33");
    }
}
