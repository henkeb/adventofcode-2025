pub fn puzzle_1(input: &str) -> String {
    let corners = handle_input(input);
    find_largest_square(&corners)
}

pub fn puzzle_2(input: &str) -> String {
    let _ = handle_input(input);
    "Not implemented yet!".to_string()
}

fn find_largest_square(corners: &Vec<(isize, isize)>) -> String {
    let mut largest = 0;
    for (i, a) in corners.iter().enumerate() {
        for b in corners.iter().skip(i + 1) {
            largest = std::cmp::max(largest, ((a.0 - b.0).abs() + 1) * ((a.1 - b.1).abs() + 1));
        }
    }
    largest.to_string()
}

fn handle_input(input: &str) -> Vec<(isize, isize)> {
    input
        .lines()
        .map(|line| line.split(',').collect::<Vec<&str>>())
        .map(|co| (co[0].parse().unwrap(), co[1].parse().unwrap()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    #[test]
    fn test_1() {
        assert_eq!(puzzle_1(&INPUT), "50");
    }

    #[test]
    fn test_2() {
        assert_eq!(puzzle_2(&INPUT), "");
    }
}
