pub fn puzzle_1(input: &str) -> String {
    let (_, mut map) = handle_input(input);
    let mut splits = 0;

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            match map[y][x] {
                '|' => splits += move_tachyon_beam(&mut map, (x, y)),
                _ => (),
            }
        }
    }
    splits.to_string()
}

fn move_tachyon_beam(map: &mut Vec<Vec<char>>, (x, y): (usize, usize)) -> usize {
    let new_y = y + 1;
    let mut splits = false;
    if new_y < map.len() {
        if map[new_y][x] == '.' {
            map[new_y][x] = '|'
        } else if map[new_y][x] == '^' {
            if x > 0 {
                if map[new_y][x - 1] == '.' {
                    map[new_y][x - 1] = '|';
                    splits = true;
                }
            }
            if x < map[0].len() - 1 {
                if map[new_y][x + 1] == '.' {
                    map[new_y][x + 1] = '|';
                    splits = true;
                }
            }
        }
    }
    if splits { 1 } else { 0 }
}

pub fn puzzle_2(input: &str) -> String {
    let (start, map) = handle_input(input);

    let mut timelines = vec![0; map[0].len()];
    timelines[start.0] = 1;

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            let timeline = timelines[x];

            if map[y][x] == '^' && timeline > 0 {
                timelines[x] = 0;
                timelines[x - 1] += timeline;
                timelines[x + 1] += timeline;
            }
        }
    }
    timelines.iter().sum::<usize>().to_string()
}

fn handle_input(input: &str) -> ((usize, usize), Vec<Vec<char>>) {
    let mut map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut start = (0, 0);
    'outer: for (i, line) in input.lines().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            if ch == 'S' {
                map[i][j] = '|';
                start = (j, i);
                break 'outer;
            }
        }
    }
    (start, map)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn test_1() {
        assert_eq!(puzzle_1(&INPUT), "21");
    }

    #[test]
    fn test_2() {
        assert_eq!(puzzle_2(&INPUT), "40");
    }
}
