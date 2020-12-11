use std::error::Error;
use std::fs;

#[derive(Clone, Debug, PartialEq)]
enum Pos {
    Floor,
    EmptySeat,
    Occupied,
}

fn print_map(map: &Vec<Vec<Pos>>) {
    for row in map {
        for p in row {
            match p {
                Pos::Floor => print!("."),
                Pos::EmptySeat => print!("L"),
                Pos::Occupied => print!("#"),
            }
        }
        println!("");
    }
    println!("");
    println!("");
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let mut map: Vec<Vec<Pos>> = input
        .lines()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| {
            s.chars()
                .map(|c| match c {
                    '.' => Pos::Floor,
                    'L' => Pos::EmptySeat,
                    '#' => Pos::Occupied,
                    _ => {
                        panic!("Illegal char: {}", c);
                    }
                })
                .collect()
        })
        .collect();

    let mut changes = 1;

    let directions: Vec<(i32, i32)> = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (1, -1),
        (1, 0),
        (1, 1),
        (0, -1),
        (0, 1),
    ];

    while changes != 0 {
        changes = 0;
        let old_map = map.clone();

        for i in 0..map.len() {
            for j in 0..map[0].len() {
                let mut occupied = 0;

                for direction in &directions {
                    let mut terminated = false;
                    let mut test_coords = (i as i32, j as i32);
                    while !terminated {
                        test_coords.0 += direction.0;
                        test_coords.1 += direction.1;
                        if test_coords.0 < 0
                            || test_coords.0 >= map.len() as i32
                            || test_coords.1 < 0
                            || test_coords.1 >= map[0].len() as i32
                        {
                            terminated = true;
                        }

                        if !terminated {
                            match old_map[test_coords.0 as usize][test_coords.1 as usize] {
                                Pos::Floor => {}
                                Pos::EmptySeat => {
                                    terminated = true;
                                }
                                Pos::Occupied => {
                                    occupied += 1;
                                    terminated = true;
                                }
                            }
                        }
                    }
                }

                match map[i][j] {
                    Pos::Floor => {}
                    Pos::EmptySeat => {
                        if occupied == 0 {
                            map[i][j] = Pos::Occupied;
                            changes += 1;
                        }
                    }
                    Pos::Occupied => {
                        if occupied >= 5 {
                            map[i][j] = Pos::EmptySeat;
                            changes += 1;
                        }
                    }
                }
            }
        }

        //print_map(&map);
    }

    let mut res = 0;

    for i in 0..map.len() {
        for j in 0..map[0].len() {
            match map[i][j] {
                Pos::Floor => {}
                Pos::EmptySeat => {}
                Pos::Occupied => {
                    res += 1;
                }
            }
        }
    }

    println!("res: {}", res);

    Ok(())
}
