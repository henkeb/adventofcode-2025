pub fn puzzle_1(input: &str) -> String {
    let mut dial = 50;
    let mut password = 0;
    for line in input.lines() {
        let (direction, rotation) = line.split_at(1);
        let mut rotation = rotation.parse::<u32>().unwrap();

        rotation = rotation % 100;

        match direction {
            "L" => {
                if rotation > dial {
                    dial = 100 - rotation + dial;
                } else {
                    dial -= rotation;
                }
            }
            "R" => {
                dial = (dial + rotation) % 100;
            }
            _ => panic!("Wrong input"),
        }
        dial %= 100;
        if dial == 0 {
            password += 1;
        }
    }
    password.to_string()
}

pub fn puzzle_2(input: &str) -> String {
    let mut dial = 50;
    let mut password = 0;
    for line in input.lines() {
        let (direction, rotation) = line.split_at(1);
        let mut rotation = rotation.parse::<u32>().unwrap();

        password += rotation / 100;

        rotation = rotation % 100;

        match direction {
            "L" => {
                if rotation > dial {
                    if dial != 0 {
                        password += 1;
                    }
                    dial = 100 - (rotation - dial);
                } else {
                    dial -= rotation;
                }
            }
            "R" => {
                dial += rotation;
                if dial > 100 {
                    password += 1;
                }
            }
            _ => panic!("Wrong input"),
        }
        dial %= 100;
        if dial == 0 {
            password += 1;
        }
    }
    password.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn test_1() {
        assert_eq!(puzzle_1(&INPUT), "3");
    }

    #[test]
    fn test_2() {
        assert_eq!(puzzle_2(&INPUT), "6");
    }
}
