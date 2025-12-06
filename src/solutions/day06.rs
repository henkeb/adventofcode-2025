pub fn puzzle_1(input: &str) -> String {
    let math_problems = handle_input(input);
    let mut output: usize = 0;
    for problem in math_problems {
        match problem.sign {
            Some(sign) => match sign {
                MathSign::Addition => output += problem.numbers.iter().sum::<usize>(),
                MathSign::Multiplication => output += problem.numbers.iter().product::<usize>(),
            },
            None => panic!("Error"),
        }
    }
    output.to_string()
}

pub fn puzzle_2(input: &str) -> String {
    let mut output = 0;
    let input: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();
    let row_len = input.len();
    let col_len = input[0].len();
    let mut numbers: Vec<usize> = Vec::new();
    for x in (0..col_len).rev() {
        let mut number: usize = 0;
        let mut math_sign = None;
        for y in 0..row_len {
            if let Some(num) = input[y][x].to_digit(10) {
                number *= 10;
                number += num as usize;
            } else if input[y][x] != ' ' {
                math_sign = Some(input[y][x]);
            }
            if y == row_len - 1 {
                if number != 0 {
                    numbers.push(number);
                }
            }
        }
        if math_sign.is_some() {
            match math_sign {
                Some('*') => output += numbers.iter().product::<usize>(),
                Some('+') => output += numbers.iter().sum::<usize>(),
                _ => (),
            }
            numbers.clear();
        }
    }
    output.to_string()
}

fn handle_input(input: &str) -> Vec<MathProblem> {
    let mut first_line = input
        .lines()
        .take(1)
        .map(|line| line.split_whitespace().count());

    let mut output: Vec<MathProblem> = vec![
        MathProblem {
            numbers: vec![],
            sign: None
        };
        first_line.next().unwrap()
    ];
    for line in input.lines() {
        for (i, split) in line.split_whitespace().enumerate() {
            match split.parse::<usize>() {
                Ok(num) => output[i].numbers.push(num),
                Err(_) => match split {
                    "*" => output[i].sign = Some(MathSign::Multiplication),
                    "+" => output[i].sign = Some(MathSign::Addition),
                    _ => panic!("Wrong sign"),
                },
            }
        }
    }
    output
}

#[derive(Clone, Debug)]
enum MathSign {
    Multiplication,
    Addition,
}

#[derive(Clone, Debug)]
struct MathProblem {
    numbers: Vec<usize>,
    sign: core::option::Option<MathSign>,
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

    #[test]
    fn test_1() {
        assert_eq!(puzzle_1(&INPUT), "4277556");
    }

    #[test]
    fn test_2() {
        assert_eq!(puzzle_2(&INPUT), "3263827");
    }
}
