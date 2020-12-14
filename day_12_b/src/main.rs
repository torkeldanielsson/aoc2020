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

    let mut ship_pos = vec![0, 0];
    let mut waypoint_pos = vec![10, 1];

    for action in &actions {
        match action {
            Action::North { steps } => {
                waypoint_pos[1] += steps;
            }
            Action::South { steps } => {
                waypoint_pos[1] -= steps;
            }
            Action::East { steps } => {
                waypoint_pos[0] += steps;
            }
            Action::West { steps } => {
                waypoint_pos[0] -= steps;
            }
            Action::Left { degrees } => match degrees {
                90 => {
                    let waypoint_pos_tmp = waypoint_pos.clone();
                    waypoint_pos[0] = -waypoint_pos_tmp[1];
                    waypoint_pos[1] = waypoint_pos_tmp[0];
                }
                180 => {
                    let waypoint_pos_tmp = waypoint_pos.clone();
                    waypoint_pos[0] = -waypoint_pos_tmp[0];
                    waypoint_pos[1] = -waypoint_pos_tmp[1];
                }
                270 => {
                    let waypoint_pos_tmp = waypoint_pos.clone();
                    waypoint_pos[0] = waypoint_pos_tmp[1];
                    waypoint_pos[1] = -waypoint_pos_tmp[0];
                }
                _ => panic!("not 90 or 180: {}", degrees),
            },
            Action::Right { degrees } => match degrees {
                90 => {
                    let waypoint_pos_tmp = waypoint_pos.clone();
                    waypoint_pos[0] = waypoint_pos_tmp[1];
                    waypoint_pos[1] = -waypoint_pos_tmp[0];
                }
                180 => {
                    let waypoint_pos_tmp = waypoint_pos.clone();
                    waypoint_pos[0] = -waypoint_pos_tmp[0];
                    waypoint_pos[1] = -waypoint_pos_tmp[1];
                }
                270 => {
                    let waypoint_pos_tmp = waypoint_pos.clone();
                    waypoint_pos[0] = -waypoint_pos_tmp[1];
                    waypoint_pos[1] = waypoint_pos_tmp[0];
                }
                _ => panic!(),
            },
            Action::Forward { steps } => {
                ship_pos[0] += steps * waypoint_pos[0];
                ship_pos[1] += steps * waypoint_pos[1];
            }
        }

        println!("ship_pos: {:?}", ship_pos);
        println!("waypoint_pos: {:?}\n", waypoint_pos);
    }

    println!("ship_pos: {:?}", ship_pos);
    println!(
        "manhattan distance: {}",
        ship_pos[0].abs() + ship_pos[1].abs()
    );

    Ok(())
}
