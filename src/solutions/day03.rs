use std::collections::HashMap;

pub fn puzzle_1(input: &str) -> String {
    const NUM_BANKS: usize = 2;
    calculate_banks_num(input, NUM_BANKS)
}

pub fn puzzle_2(input: &str) -> String {
    const NUM_BANKS: usize = 12;
    calculate_banks_num(input, NUM_BANKS)
}
fn calculate_banks_num(input: &str, num_banks: usize) -> String {
    input
        .lines()
        .fold(0, |sum, line| {
            let mut output: Vec<char> = vec!['0'; num_banks];
            let line: Vec<char> = line.chars().collect();
            let mut range: (usize, usize) = (0, line.len() - num_banks);

            for current_bank in 0..num_banks {
                let mut h_map: HashMap<char, usize> = HashMap::new();
                for pos in range.0..=range.1 {
                    h_map.entry(line[pos]).or_insert(pos);
                }
                for digit in ('1'..='9').rev() {
                    match h_map.get(&digit) {
                        Some(pos) => {
                            output[current_bank] = digit;
                            range.0 = pos + 1;
                            range.1 += 1;
                            break;
                        }
                        None => (),
                    }
                }
            }
            sum + output
                .into_iter()
                .collect::<String>()
                .parse::<u128>()
                .unwrap_or(0)
        })
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn test_1() {
        assert_eq!(puzzle_1(&INPUT), "357");
    }

    #[test]
    fn test_2() {
        assert_eq!(puzzle_2(&INPUT), "3121910778619");
    }
}
