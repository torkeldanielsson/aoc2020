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

    println!("{:?}", map);

    println!("size: {}", map.len());

    Ok(())
}
