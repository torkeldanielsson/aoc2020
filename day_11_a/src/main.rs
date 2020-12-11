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

    while changes != 0 {
        changes = 0;
        let old_map = map.clone();

        for i in 0..map.len() {
            for j in 0..map[0].len() {
                let mut occupied = 0;

                for i_adj in (i as i32) - 1..(i as i32) + 2 {
                    for j_adj in (j as i32) - 1..(j as i32) + 2 {
                        if i_adj >= 0
                            && i_adj < map.len() as i32
                            && j_adj >= 0
                            && j_adj < map[0].len() as i32
                            && !(i_adj == i as i32 && j_adj == j as i32)
                        {
                            if old_map[i_adj as usize][j_adj as usize] == Pos::Occupied {
                                occupied += 1;
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
                        if occupied >= 4 {
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
