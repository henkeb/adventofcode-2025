pub fn puzzle_1(input: &str) -> String {
    let (ranges, numbers) = handle_input(input);
    let mut output = 0;

    'outer: for num in numbers.into_iter() {
        for range in ranges.iter() {
            if num >= range.0 && num <= range.1 {
                output += 1;
                continue 'outer;
            }
        }
    }

    output.to_string()
}

pub fn puzzle_2(input: &str) -> String {
    let (ranges, _) = handle_input(input);
    let sum: usize = ranges.iter().map(|(a, b)| b - a + 1).sum();
    sum.to_string()
}

fn handle_input(input: &str) -> (Vec<(usize, usize)>, Vec<usize>) {
    let mut it = input.split("\n\n");
    let ranges = it.next().unwrap();
    let numbers = it.next().unwrap();

    let mut ranges: Vec<(usize, usize)> = ranges
        .lines()
        .map(|line| match line.split_once("-") {
            Some((a, b)) => (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap()),
            None => panic!("wrong"),
        })
        .collect();

    ranges.sort();

    let mut output_ranges: Vec<(usize, usize)> = Vec::new();

    output_ranges.push(ranges[0]);

    for i in 1..ranges.len() {
        let curr = ranges[i];
        if let Some(mut last) = output_ranges.pop_if(|last| curr.0 <= last.1) {
            last.1 = std::cmp::max(last.1, curr.1);
            output_ranges.push(last);
        } else {
            output_ranges.push(curr);
        }
    }

    (
        output_ranges,
        numbers.lines().map(|line| line.parse().unwrap()).collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn test_1() {
        assert_eq!(puzzle_1(&INPUT), "3");
    }

    #[test]
    fn test_2() {
        assert_eq!(puzzle_2(&INPUT), "14");
    }
}
