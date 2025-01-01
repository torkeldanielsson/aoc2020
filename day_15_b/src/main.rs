use std::{error::Error, time::Instant};

fn solve_memory_game(input: &str, target_turn: usize) -> i32 {
    let starting_numbers: Vec<i32> = input
        .trim()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();

    let mut spoken_numbers = vec![-1; target_turn];

    for (i, &num) in starting_numbers
        .iter()
        .take(starting_numbers.len() - 1)
        .enumerate()
    {
        spoken_numbers[num as usize] = i as i32;
    }

    let mut last_number = *starting_numbers.last().unwrap();

    for turn in (starting_numbers.len() - 1)..target_turn - 1 {
        let next_number = if spoken_numbers[last_number as usize] == -1 {
            0
        } else {
            (turn as i32) - spoken_numbers[last_number as usize]
        };

        spoken_numbers[last_number as usize] = turn as i32;
        last_number = next_number;
    }

    last_number
}

fn main() -> Result<(), Box<dyn Error>> {
    let t = Instant::now();

    let input = include_str!("../input");
    let res = solve_memory_game(input, 30000000);

    println!("res: {res}, {} us", t.elapsed().as_micros());

    Ok(())
}
