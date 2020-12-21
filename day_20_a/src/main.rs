use std::collections::HashMap;
use std::error::Error;
use std::fs;

#[derive(Clone, Debug, PartialEq)]
struct Tile {
    Id: i32,
    Sides: Vec<i32>,
}

fn bit_reverse(n: i32) -> i32 {
    let mut res = 0;
    for i in 0..10 {
        if (n & (1 << i)) != 0 {
            res |= 1 << (9 - i);
        }
    }
    res
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let mut side_to_tile_map: HashMap<i32, Vec<i32>> = HashMap::new();

    let mut tiles = Vec::new();

    for input_tile in input
        .split("\n\n")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
    {
        let input_tile_lines: Vec<&str> = input_tile.lines().collect();
        let number = input_tile_lines[0][5..9].parse::<i32>().unwrap();
        let mut map = Vec::new();
        for line in &input_tile_lines[1..] {
            let mut map_line = Vec::new();
            for c in line.chars() {
                match c {
                    '#' => map_line.push(1),
                    '.' => map_line.push(0),
                    _ => panic!(),
                }
            }
            map.push(map_line);
        }

        let mut top = 0;
        let mut bottom = 0;
        let mut left = 0;
        let mut right = 0;

        for i in 0..10 {
            if map[0][i] == 1 {
                top |= 1 << i
            }
            if map[9][i] == 1 {
                bottom |= 1 << i
            }
            if map[i][0] == 1 {
                left |= 1 << i
            }
            if map[i][9] == 1 {
                right |= 1 << i
            }
        }

        for side in &[&top, &bottom, &left, &right] {
            if side_to_tile_map.contains_key(side) {
                side_to_tile_map.get_mut(*side).unwrap().push(number);
            } else {
                side_to_tile_map.insert(**side, vec![number]);
            }
            let reverse = bit_reverse(**side);
            if side_to_tile_map.contains_key(&reverse) {
                side_to_tile_map.get_mut(&reverse).unwrap().push(number);
            } else {
                side_to_tile_map.insert(reverse, vec![number]);
            }
        }

        tiles.push(Tile {
            Id: number,
            Sides: vec![top, bottom, left, right],
        });

        // println!("{:010b}", top);
    }

    println!("{:?}", side_to_tile_map);
    println!("{:?}", tiles);

    let mut res: i64 = 1;

    for (a, b) in &[(0, 2), (0, 3), (1, 2), (1, 3)] {
        for tile in &tiles {
            if side_to_tile_map[&tile.Sides[*a as usize]].len() == 1
                && side_to_tile_map[&tile.Sides[*b as usize]].len() == 1
            {
                res *= tile.Id as i64;
                println!("got corner {:?}", (a, b));
            }
        }
    }

    println!("res {:?}", res);

    Ok(())
}
