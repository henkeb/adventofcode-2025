pub fn puzzle_1(input: &str) -> String {
    let corners = handle_input(input);
    find_largest_rectangle(&corners)
}

pub fn puzzle_2(input: &str) -> String {
    let corners = handle_input(input);
    let length = corners.len();
    let rectangles = rectangles(&corners);

    let (a, b) = rectangles
        .iter()
        .find(|(a, b)| {
            let (x_min, x_max, y_min, y_max) = edges(a, b);
            for (i, corner_1) in corners.iter().enumerate() {
                let corner_2 = &corners[(i + 1) % length];

                if corner_1.0 == corner_2.0 {
                    let (y_lmin, y_lmax) = (corner_1.1.min(corner_2.1), corner_1.1.max(corner_2.1));
                    if x_min < corner_1.0
                        && x_max > corner_1.0
                        && !(y_min >= y_lmax || y_max <= y_lmin)
                    {
                        return false;
                    }
                } else if corner_1.1 == corner_2.1 {
                    let (x_lmin, x_lmax) = (corner_1.0.min(corner_2.0), corner_1.0.max(corner_2.0));
                    if y_min < corner_1.1
                        && y_max > corner_1.1
                        && !(x_min >= x_lmax || x_max <= x_lmin)
                    {
                        return false;
                    }
                } else {
                    panic!("Unreachable");
                }
            }
            true
        })
        .unwrap();

    area(*a, *b).to_string()
}

fn edges(point1: &(isize, isize), point2: &(isize, isize)) -> (isize, isize, isize, isize) {
    (
        point1.0.min(point2.0),
        point1.0.max(point2.0),
        point1.1.min(point2.1),
        point1.1.max(point2.1),
    )
}

fn rectangles(coords: &[(isize, isize)]) -> Vec<((isize, isize), (isize, isize))> {
    let mut pairs = vec![];
    for (i, &a) in coords.iter().enumerate() {
        for &b in coords.iter().skip(i) {
            pairs.push((a, b));
        }
    }

    pairs.sort_by(|a, b| area(b.0, b.1).cmp(&area(a.0, a.1)));
    pairs
}

fn area(a: (isize, isize), b: (isize, isize)) -> isize {
    ((a.0 - b.0).abs() + 1) * ((a.1 - b.1).abs() + 1)
}

fn find_largest_rectangle(corners: &Vec<(isize, isize)>) -> String {
    let mut largest = 0;
    for (i, a) in corners.iter().enumerate() {
        for b in corners.iter().skip(i + 1) {
            largest = std::cmp::max(largest, area(*a, *b));
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
        assert_eq!(puzzle_2(&INPUT), "24");
    }
}
