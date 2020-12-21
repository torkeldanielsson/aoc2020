use std::collections::HashMap;
use std::error::Error;
use std::fs;

#[derive(Clone, Debug, PartialEq)]
struct Tile {
    Sides: Vec<i32>,
    Map: Vec<Vec<i32>>,
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

#[derive(Clone, Debug, PartialEq)]
enum Transform {
    Rotate,
    Flip,
}

fn transform_tile(transform: Transform, tile: &mut Tile) {
    let input = tile.clone();
    match transform {
        Transform::Flip => {
            for i in 0..10 {
                tile.Map[i] = input.Map[9 - i].clone();
            }
            tile.Sides[0] = input.Sides[1];
            tile.Sides[1] = input.Sides[0];
        }
        Transform::Rotate => {
            for i in 0..10 {
                for j in 0..10 {
                    tile.Map[i][j] = input.Map[j][9 - i];
                }
            }
            tile.Sides[0] = input.Sides[3];
            tile.Sides[1] = input.Sides[2];
            tile.Sides[2] = input.Sides[0];
            tile.Sides[3] = input.Sides[1];
        }
    }
}

fn print_tile(tile: &Tile) {
    for y in 0..10 {
        for x in 0..10 {
            print!("{}", if tile.Map[y][x] == 0 { "." } else { "#" });
        }
        println!();
    }
    println!();
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let mut side_to_tile_map: HashMap<i32, Vec<i32>> = HashMap::new();

    let mut tiles: HashMap<i32, Tile> = HashMap::new();

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

        tiles.insert(
            number,
            Tile {
                Map: map,
                Sides: vec![top, bottom, left, right],
            },
        );

        // println!("{:010b}", top);
    }

    // println!("{:?}", side_to_tile_map);
    // println!("{:?}", tiles);

    let mut top_left_tile = 0;

    for tile in &tiles {
        if side_to_tile_map[&tile.1.Sides[0]].len() == 1
            && side_to_tile_map[&tile.1.Sides[2]].len() == 1
        {
            top_left_tile = *tile.0;
        }
    }

    let mut id_map = Vec::new();
    id_map.push(vec![top_left_tile]);

    {
        let mut x = 0;

        loop {
            let last_id = id_map[0][x];

            x += 1;

            let neighbors = &side_to_tile_map[&tiles[&last_id].Sides[3]];
            let next_tile_id = if neighbors[0] == last_id {
                neighbors[1]
            } else {
                neighbors[0]
            };

            id_map[0].push(next_tile_id);

            for transform in &[
                Transform::Rotate,
                Transform::Rotate,
                Transform::Rotate,
                Transform::Rotate,
                Transform::Flip,
                Transform::Rotate,
                Transform::Rotate,
                Transform::Rotate,
            ] {
                transform_tile(transform.clone(), tiles.get_mut(&next_tile_id).unwrap());
                let mut is_ok = true;
                for i in 0..10 {
                    if tiles[&next_tile_id].Map[i][0] != tiles[&last_id].Map[i][9] {
                        is_ok = false;
                        break;
                    }
                }
                if is_ok {
                    break;
                }
            }

            assert_eq!(side_to_tile_map[&tiles[&next_tile_id].Sides[0]].len(), 1);

            if side_to_tile_map[&tiles[&next_tile_id].Sides[3]].len() == 1 {
                break;
            }
        }
    }

    for x in 0..id_map[0].len() {
        let mut y = 0;

        loop {
            let last_id = id_map[y][x];

            y += 1;

            let neighbors = &side_to_tile_map[&tiles[&last_id].Sides[1]];
            let next_tile_id = if neighbors[0] == last_id {
                neighbors[1]
            } else {
                neighbors[0]
            };

            if x == 0 {
                id_map.push(Vec::new());
            }
            id_map[y].push(next_tile_id);

            for transform in &[
                Transform::Rotate,
                Transform::Rotate,
                Transform::Rotate,
                Transform::Rotate,
                Transform::Flip,
                Transform::Rotate,
                Transform::Rotate,
                Transform::Rotate,
            ] {
                transform_tile(transform.clone(), tiles.get_mut(&next_tile_id).unwrap());
                let mut is_ok = true;
                for i in 0..10 {
                    if tiles[&next_tile_id].Map[0][i] != tiles[&last_id].Map[9][i] {
                        is_ok = false;
                        break;
                    }
                }
                if is_ok {
                    break;
                }
            }

            if side_to_tile_map[&tiles[&next_tile_id].Sides[1]].len() == 1 {
                break;
            }
        }
    }

    let mut map: Vec<Vec<i32>> = Vec::new();
    for y in 0..12 {
        for x in 0..12 {
            let tile = &tiles[&id_map[y][x]];
            for ty in 1..9 {
                if x == 0 {
                    map.push(Vec::new());
                }

                for tx in 1..9 {
                    map[y * 8 + ty - 1].push(tile.Map[ty][tx]);
                }
            }
        }
    }

    let mut sea_monsters = 0;

    for transform in &[
        Transform::Rotate,
        Transform::Rotate,
        Transform::Rotate,
        Transform::Rotate,
        Transform::Flip,
        Transform::Rotate,
        Transform::Rotate,
        Transform::Rotate,
    ] {
        let input = map.clone();

        match transform {
            Transform::Flip => {
                for i in 0..96 {
                    map[i] = input[95 - i].clone();
                }
            }
            Transform::Rotate => {
                for i in 0..96 {
                    for j in 0..96 {
                        map[i][j] = input[j][95 - i];
                    }
                }
            }
        }

        for (y, line) in map.iter().enumerate() {
            for (x, n) in line.iter().enumerate() {
                if x + 19 < line.len() && y + 2 < map.len() {
                    if map[y + 0][x + 18] == 1
                        && map[y + 1][x + 0] == 1
                        && map[y + 1][x + 5] == 1
                        && map[y + 1][x + 6] == 1
                        && map[y + 1][x + 11] == 1
                        && map[y + 1][x + 12] == 1
                        && map[y + 1][x + 17] == 1
                        && map[y + 1][x + 18] == 1
                        && map[y + 1][x + 19] == 1
                        && map[y + 2][x + 1] == 1
                        && map[y + 2][x + 4] == 1
                        && map[y + 2][x + 7] == 1
                        && map[y + 2][x + 10] == 1
                        && map[y + 2][x + 13] == 1
                        && map[y + 2][x + 16] == 1
                    {
                        sea_monsters += 1;
                    }
                }

                if *n == 0 {
                    //print!(".")
                } else {
                    //print!("#")
                }
            }
            //println!()
        }
    }

    let mut total = 0;

    for line in map {
        for n in line {
            if n == 1 {
                total += 1;
            }
        }
    }

    println!("res: {}", total - sea_monsters * 15);

    Ok(())
}
