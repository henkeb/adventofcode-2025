pub fn puzzle_1(input: &str) -> String {
    let (pieces, map_data) = handle_input(input);
    let used_space_per_piece: Vec<usize> = pieces
        .iter()
        .map(|p| {
            let mut count = 0;
            for line in p.iter() {
                for ch in line {
                    if *ch == '#' {
                        count += 1;
                    }
                }
            }
            count
        })
        .collect();

    let mut regions = 0;

    for (width, length, pieces) in map_data {
        let total_piece_area = pieces
            .iter()
            .enumerate()
            .fold(0, |piece_area, (i, num_pieces)| {
                piece_area + used_space_per_piece[i] * num_pieces
            });

        if total_piece_area <= width * length {
            regions += 1;
        }
    }

    regions.to_string()
}

pub fn puzzle_2(_input: &str) -> String {
    "god jul".to_string()
}

fn handle_input(input: &str) -> (Vec<Vec<Vec<char>>>, Vec<(usize, usize, Vec<usize>)>) {
    let splits: Vec<&str> = input.split("\n\n").collect();
    let len_splits = splits.len();
    let map_data: Vec<(usize, usize, Vec<usize>)> = splits[len_splits - 1]
        .lines()
        .map(|line| {
            let line_split: Vec<&str> = line.split(": ").collect();
            let (width, length) = line_split[0].split_once("x").expect("formatting");
            let width: usize = width.parse().unwrap();
            let length: usize = length.parse().unwrap();
            let piecies: Vec<usize> = line_split[1]
                .split_whitespace()
                .filter_map(|val| val.parse::<usize>().ok())
                .collect();
            (width, length, piecies)
        })
        .collect();

    let mut pieces: Vec<Vec<Vec<char>>> = Vec::new();

    for &piece_data in &splits[0..len_splits - 1] {
        let mut piece: Vec<Vec<char>> = Vec::new();
        piece_data.lines().skip(1).for_each(|line| {
            piece.push(line.chars().collect());
        });
        pieces.push(piece);
    }

    (pieces, map_data)
}
