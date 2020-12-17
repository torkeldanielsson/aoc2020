use std::collections::HashSet;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let mut grid: HashSet<(i32, i32, i32)> = HashSet::new();

    for (y, line) in input
        .lines()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .enumerate()
    {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                grid.insert((x as i32, y as i32, 0));
            }
        }
    }

    let mut add_positions = Vec::new();
    for x in -1..2 {
        for y in -1..2 {
            for z in -1..2 {
                add_positions.push((x, y, z));
            }
        }
    }

    let mut check_positions = Vec::new();
    for x in -1..2 {
        for y in -1..2 {
            for z in -1..2 {
                if !(x == 0 && y == 0 && z == 0) {
                    check_positions.push((x, y, z));
                }
            }
        }
    }

    for _ in 0..6 {
        let mut new_grid: HashSet<(i32, i32, i32)> = HashSet::new();
        let mut positions_to_check = HashSet::new();

        for pos in &grid {
            for add in &add_positions {
                positions_to_check.insert((pos.0 + add.0, pos.1 + add.1, pos.2 + add.2));
            }
        }

        for pos in &positions_to_check {
            let mut neighbors = 0;
            for check_add in &check_positions {
                if grid.contains(&(
                    pos.0 + check_add.0,
                    pos.1 + check_add.1,
                    pos.2 + check_add.2,
                )) {
                    neighbors += 1;
                }
            }

            if grid.contains(pos) {
                if neighbors == 2 || neighbors == 3 {
                    new_grid.insert(*pos);
                }
            } else if neighbors == 3 {
                new_grid.insert(*pos);
            }
        }

        grid = new_grid;
    }

    println!("size: {}", grid.len());

    Ok(())
}
