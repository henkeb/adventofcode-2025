pub fn puzzle_1(input: &str) -> String {
    let mut map = handle_input(input);
    count_paper_and_remove(&mut map, true).to_string()
}

pub fn puzzle_2(input: &str) -> String {
    let mut map = handle_input(input);
    let mut total_paper_count = 0;
    loop {
        let count = count_paper_and_remove(&mut map, false);
        if count == 0 {
            break;
        }
        total_paper_count += count;
    }
    total_paper_count.to_string()
}

fn count_paper_and_remove(map: &mut Vec<Vec<char>>, only_count: bool) -> usize {
    let row_len = map.len() as isize;
    let col_len = map[0].len() as isize;
    let mut total_paper_count = 0;
    let mut paper_positions_to_take: Vec<(usize, usize)> = Vec::new();
    for y in 0..map.len() {
        let y_isize = y as isize;
        for x in 0..map[0].len() {
            let mut paper_count = 0;
            if map[y][x] == '@' {
                let x_isize = x as isize;
                for dir in DIRECTIONS.iter() {
                    let new_pos = (x_isize + dir.0, y_isize + dir.1);
                    if new_pos.0 >= 0 && new_pos.0 < col_len {
                        if new_pos.1 >= 0 && new_pos.1 < row_len {
                            if map[new_pos.1 as usize][new_pos.0 as usize] == '@' {
                                paper_count += 1;
                            }
                        }
                    }
                }
                if paper_count < 4 {
                    total_paper_count += 1;
                    paper_positions_to_take.push((x, y));
                }
            }
        }
    }
    if !only_count {
        for (x, y) in paper_positions_to_take.into_iter() {
            map[y][x] = '.';
        }
    }
    total_paper_count
}

fn handle_input(input: &str) -> Vec<Vec<char>> {
    let mut map: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        map.push(line.chars().collect());
    }
    map
}

const DIRECTIONS: [(isize, isize); 8] = [
    (-1, 1),
    (0, 1),
    (1, 1),
    (-1, 0),
    (1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
];

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn test_1() {
        assert_eq!(puzzle_1(&INPUT), "13");
    }

    #[test]
    fn test_2() {
        assert_eq!(puzzle_2(&INPUT), "43");
    }
}
