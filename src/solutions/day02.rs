pub fn puzzle_1(input: &str) -> String {
    let mut sum = 0;
    input
        .replace(",\n", ",")
        .replace("\n", "")
        .split(',')
        .for_each(|line| {
            let range: Vec<&str> = line.split('-').collect();
            sum += sum_of_repeated_num_in_range(
                range[0].parse::<u128>().unwrap(),
                range[1].parse::<u128>().unwrap(),
                true,
            );
        });
    sum.to_string()
}

pub fn puzzle_2(input: &str) -> String {
    let mut sum = 0;
    input
        .replace(",\n", ",")
        .replace("\n", "")
        .split(',')
        .for_each(|line| {
            let range: Vec<&str> = line.split('-').collect();
            sum += sum_of_repeated_num_in_range(
                range[0].parse::<u128>().unwrap(),
                range[1].parse::<u128>().unwrap(),
                false,
            );
        });
    sum.to_string()
}

fn sum_of_repeated_num_in_range(low: u128, high: u128, first: bool) -> u128 {
    (low..=high).fold(0, |sum, num| {
        if is_repeated_num(&num, first) {
            sum + num
        } else {
            sum
        }
    })
}

fn is_repeated_num(num: &u128, first: bool) -> bool {
    let digits = (num.checked_ilog10().unwrap_or(0) + 1) as usize;

    let num_string: Vec<char> = num.to_string().chars().collect();
    if first {
        // check if even number of digits
        if digits & 1 == 0 {
            let (left, right) = num_string.split_at((digits / 2) as usize);
            left == right
        } else {
            false
        }
    } else {
        for i in 1..=(digits / 2) {
            if digits % i == 0 {
                let d_slices: Vec<&[char]> = num_string.chunks(i).collect();
                let equal = d_slices
                    .iter()
                    .skip(1)
                    .all(|chunk| d_slices.first().unwrap() == chunk);
                if equal {
                    return true;
                }
            }
        }
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
