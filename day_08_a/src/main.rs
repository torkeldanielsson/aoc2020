use std::error::Error;
use std::fs;

#[derive(Debug, PartialEq)]
enum Instruction {
    Acc { value: i64 },
    Jmp { value: i64 },
    Nop,
}

fn parse_instruction(s: &str) -> Instruction {
    let parts: Vec<&str> = s
        .split(" ")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();
    assert!(parts.len() == 2);

    let value = parts[1].parse::<i64>().unwrap();

    match parts[0] {
        "acc" => Instruction::Acc { value },
        "jmp" => Instruction::Jmp { value },
        "nop" => Instruction::Nop,
        _ => {
            panic!("Illegal instruction: {}", s);
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let mut program: Vec<(Instruction, i32, i32)> = input
        .lines()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| (parse_instruction(s), 0, 0))
        .collect();

    let mut accumulator = 0;
    let mut pc = 0;
    let mut counter = 1;

    loop {
        if program[pc as usize].2 == 1 {
            break;
        }
        program[pc as usize].2 += 1;

        program[pc as usize].1 = counter;
        counter += 1;

        match program[pc as usize].0 {
            Instruction::Acc { value } => {
                accumulator += value;
                pc += 1;
            }
            Instruction::Jmp { value } => {
                pc += value;
            }
            Instruction::Nop => {
                pc += 1;
            }
        }
    }

    for instruction in program {
        println!("{:?}", instruction);
    }

    println!("accumulator: {}", accumulator);

    Ok(())
}
