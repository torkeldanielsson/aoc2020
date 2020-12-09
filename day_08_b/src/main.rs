use std::error::Error;
use std::fs;

#[derive(Clone, Debug, PartialEq)]
enum Instruction {
    Acc { value: i64 },
    Jmp { value: i64 },
    Nop { value: i64 },
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
        "nop" => Instruction::Nop { value },
        _ => {
            panic!("Illegal instruction: {}", s);
        }
    }
}

fn run_program(mut program: Vec<(Instruction, i32, i32)>) -> Option<i64> {
    let mut accumulator: i64 = 0;
    let mut pc: i64 = 0;
    let mut counter = 1;

    loop {
        if pc < 0 {
            break;
        }

        if pc >= program.len() as i64 {
            return Some(accumulator);
        }

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
            Instruction::Nop { value: _ } => {
                pc += 1;
            }
        }
    }

    return None;
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let program: Vec<(Instruction, i32, i32)> = input
        .lines()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| (parse_instruction(s), 0, 0))
        .collect();

    for i in 0..program.len() {
        let mut program_copy = program.clone();
        let mut do_run = false;

        match program_copy[i].0 {
            Instruction::Nop { value } => {
                program_copy[i].0 = Instruction::Jmp { value };
                do_run = true;
            }
            Instruction::Jmp { value } => {
                program_copy[i].0 = Instruction::Nop { value };
                do_run = true;
            }
            _ => {}
        }

        if do_run {
            if let Some(res) = run_program(program_copy) {
                println!("success, accumulator: {}", res);
            }
        }
    }

    Ok(())
}
