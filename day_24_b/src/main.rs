use std::collections::HashSet;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let mut map = HashSet::new();

    for line in input.lines() {
        let mut coords = (0, 0, 0);
        let chars: Vec<char> = line.chars().collect();
        let mut i = 0;
        while i < chars.len() {
            match chars[i] {
                'e' => {
                    coords.0 += 1;
                    coords.1 -= 1;
                }
                's' => {
                    i += 1;
                    match chars[i] {
                        'e' => {
                            coords.1 -= 1;
                            coords.2 += 1;
                        }
                        'w' => {
                            coords.0 -= 1;
                            coords.2 += 1;
                        }
                        _ => panic!(),
                    }
                }
                'w' => {
                    coords.0 -= 1;
                    coords.1 += 1;
                }
                'n' => {
                    i += 1;
                    match chars[i] {
                        'e' => {
                            coords.0 += 1;
                            coords.2 -= 1;
                        }
                        'w' => {
                            coords.1 += 1;
                            coords.2 -= 1;
                        }
                        _ => panic!(),
                    }
                }
                _ => panic!(),
            }

            i += 1;
        }

        if map.contains(&coords) {
            map.remove(&coords);
        } else {
            map.insert(coords);
        }
    }

    for i in 0..100 {
        let mut tiles_to_test = HashSet::new();

        for t in &map {
            for n in &[
                (0, 0, 0),
                (0, 1, -1),
                (1, 0, -1),
                (1, -1, 0),
                (0, -1, 1),
                (-1, 0, 1),
                (-1, 1, 0),
            ] {
                tiles_to_test.insert((t.0 + n.0, t.1 + n.1, t.2 + n.2));
            }
        }

        let mut new_map = HashSet::new();

        for tile in tiles_to_test {
            let mut black_tiles = 0;
            for n in &[
                (0, 1, -1),
                (1, 0, -1),
                (1, -1, 0),
                (0, -1, 1),
                (-1, 0, 1),
                (-1, 1, 0),
            ] {
                if map.contains(&(tile.0 + n.0, tile.1 + n.1, tile.2 + n.2)) {
                    black_tiles += 1;
                }
            }

            if map.contains(&tile) {
                if black_tiles == 1 || black_tiles == 2 {
                    new_map.insert(tile);
                }
            } else {
                if black_tiles == 2 {
                    new_map.insert(tile);
                }
            }
        }

        map = new_map;

        if i < 10 || (i + 1) % 10 == 0 {
            println!("day {}: {}", i + 1, map.len());
        }
    }

    Ok(())
}
