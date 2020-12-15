use std::collections::HashMap;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    /*
    mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
    mem[8] = 11
    mem[7] = 101
    mem[8] = 0
    */

    let mut or_mask: u64 = 0;
    let mut and_mask: u64 = 0xFFFFFFFFFFFFFFFF;

    let mut memory = HashMap::new();

    for line in input.lines().map(|s| s.trim()).filter(|s| !s.is_empty()) {
        match &line[0..2] {
            "ma" => {
                or_mask = 0;
                and_mask = 0xFFFFFFFFFFFFFFFF;
                for (i, c) in line.split(" = ").nth(1).unwrap().chars().enumerate() {
                    match c {
                        'X' => {}
                        '1' => {
                            or_mask |= 1 << (35 - i);
                        }
                        '0' => {
                            and_mask &= 0xFFFFFFFFFFFFFFFF ^ (1 << (35 - i));
                        }
                        _ => panic!(),
                    }
                }
            }
            "me" => {
                let address = (&line[4..])
                    .split("]")
                    .next()
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();
                let value = line.split(" = ").nth(1).unwrap().parse::<u64>().unwrap();
                memory.insert(address, (value | or_mask) & and_mask);
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
