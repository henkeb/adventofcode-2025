pub fn puzzle_1(input: &str) -> String {
    let mut sum = 0;
    input
        .replace(",\n", ",")
        .replace("\n", "")
        .split(',')
        .for_each(|line| {
            let range: Vec<&str> = line.split('-').collect();
            sum += sum_of_repeaded_num_in_range(
                range[0].parse::<u128>().unwrap(),
                range[1].parse::<u128>().unwrap(),
            );
        });
    sum.to_string()
}

pub fn puzzle_2(input: &str) -> String {
    "Not implemented yet!".to_string()
}

fn sum_of_repeaded_num_in_range(low: u128, high: u128) -> u128 {
    let mut sum = 0;
    for i in low..=high {
        if repeated_num(&i) {
            sum += i;
        }
    }
    sum
}

fn repeated_num(num: &u128) -> bool {
    let digits = num.checked_ilog10().unwrap_or(0) + 1;

    let num_string = num.to_string();

    // check if even number of digits
    if digits & 1 == 0 {
        let (left, right) = num_string.split_at((digits / 2) as usize);
        left == right
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_1() {
        assert_eq!(puzzle_1(&INPUT), "1227775554");
    }

    #[test]
    fn test_2() {
        assert_eq!(puzzle_2(&INPUT), "4174379265");
    }
}
