use std::error::Error;
use std::fs;

#[derive(Clone, Debug, PartialEq)]
enum Action {
    North { steps: i32 },
    South { steps: i32 },
    East { steps: i32 },
    West { steps: i32 },
    Left { degrees: i32 },
    Right { degrees: i32 },
    Forward { steps: i32 },
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let mut actions: Vec<Action> = Vec::new();

    for line in input.lines().map(|s| s.trim()).filter(|s| !s.is_empty()) {
        let val = line[1..].parse::<i32>().unwrap();

        actions.push(match &line[0..1] {
            "N" => Action::North { steps: val },
            "S" => Action::South { steps: val },
            "E" => Action::East { steps: val },
            "W" => Action::West { steps: val },
            "L" => Action::Left { degrees: val },
            "R" => Action::Right { degrees: val },
            "F" => Action::Forward { steps: val },
            _ => panic!(),
        });
    }

    let mut pos = vec![0, 0];
    let mut direction = vec![0, 1];

    for action in &actions {
        match action {
            Action::North { steps } => {
                pos[0] += steps;
            }
            Action::South { steps } => {
                pos[0] -= steps;
            }
            Action::East { steps } => {
                pos[1] += steps;
            }
            Action::West { steps } => {
                pos[1] -= steps;
            }
            Action::Left { degrees } => match degrees {
                90 => {
                    direction = match direction.as_slice() {
                        [1, 0] => vec![0, -1],
                        [-1, 0] => vec![0, 1],
                        [0, 1] => vec![1, 0],
                        [0, -1] => vec![-1, 0],
                        _ => panic!(),
                    }
                }
                180 => {
                    direction = match direction.as_slice() {
                        [1, 0] => vec![-1, 0],
                        [-1, 0] => vec![1, 0],
                        [0, 1] => vec![0, -1],
                        [0, -1] => vec![0, 1],
                        _ => panic!(),
                    }
                }
                270 => {
                    direction = match direction.as_slice() {
                        [1, 0] => vec![0, 1],
                        [-1, 0] => vec![0, -1],
                        [0, 1] => vec![-1, 0],
                        [0, -1] => vec![1, 0],
                        _ => panic!(),
                    }
                }
                _ => panic!("not 90 or 180: {}", degrees),
            },
            Action::Right { degrees } => match degrees {
                90 => {
                    direction = match direction.as_slice() {
                        [1, 0] => vec![0, 1],
                        [-1, 0] => vec![0, -1],
                        [0, 1] => vec![-1, 0],
                        [0, -1] => vec![1, 0],
                        _ => panic!(),
                    }
                }
                180 => {
                    direction = match direction.as_slice() {
                        [1, 0] => vec![-1, 0],
                        [-1, 0] => vec![1, 0],
                        [0, 1] => vec![0, -1],
                        [0, -1] => vec![0, 1],
                        _ => panic!(),
                    }
                }
                270 => {
                    direction = match direction.as_slice() {
                        [1, 0] => vec![0, -1],
                        [-1, 0] => vec![0, 1],
                        [0, 1] => vec![1, 0],
                        [0, -1] => vec![-1, 0],
                        _ => panic!(),
                    }
                }
                _ => panic!(),
            },
            Action::Forward { steps } => {
                pos[0] += steps * direction[0];
                pos[1] += steps * direction[1];
            }
        }
    }

    println!("pos: {:?}", pos);
    println!("res: {}", pos[0].abs() + pos[1].abs());

    Ok(())
}
