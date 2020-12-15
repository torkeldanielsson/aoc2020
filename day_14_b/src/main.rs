use std::collections::HashMap;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    /*
    mask = 000000000000000000000000000000X1001X
    mem[42] = 100
    mask = 00000000000000000000000000000000X0XX
    mem[26] = 1
    */

    let mut or_mask: u64 = 0;
    let mut flip_positions = Vec::new();

    let mut memory = HashMap::new();

    for line in input.lines().map(|s| s.trim()).filter(|s| !s.is_empty()) {
        match &line[0..2] {
            "ma" => {
                or_mask = 0;
                flip_positions = Vec::new();
                for (i, c) in line.split(" = ").nth(1).unwrap().chars().enumerate() {
                    match c {
                        'X' => {
                            flip_positions.push(i);
                        }
                        '1' => {
                            or_mask |= 1 << (35 - i);
                        }
                        '0' => {}
                        _ => panic!(),
                    }
                }
            }
            "me" => {
                let mut address = (&line[4..])
                    .split("]")
                    .next()
                    .unwrap()
                    .parse::<u64>()
                    .unwrap();
                let value = line.split(" = ").nth(1).unwrap().parse::<u64>().unwrap();
                address = address | or_mask;
                for bit_pattern in 0..(2 as u64).pow(flip_positions.len() as u32) {
                    for (i, flip_pos) in flip_positions.iter().enumerate() {
                        address &= 0xFFFFFFFFFFFFFFFF ^ (1 << (35 - flip_pos));
                        let bit_val = (bit_pattern >> i) & 0x1;
                        address |= bit_val << (35 - flip_pos);
                    }
                    // println!("{:#036b}", bit_pattern);
                    // println!("{:#036b}", address);
                    // println!("address: {}\n", address);
                    memory.insert(address, value);
                }
            }
            _ => panic!(),
        }
    }

    let mut sum = 0;
    for v in memory {
        sum += v.1;
    }

    println!("sum: {:?}", sum);

    Ok(())
}
