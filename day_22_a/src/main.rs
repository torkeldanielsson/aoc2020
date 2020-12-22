use std::collections::VecDeque;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let player_parts: Vec<&str> = input.split("\n\n").collect();
    let player_1_raw = player_parts[0];
    let player_2_raw = player_parts[1];

    let mut player_1: VecDeque<i32> = VecDeque::new();
    let mut player_2: VecDeque<i32> = VecDeque::new();

    for (raw, player) in &mut [(player_1_raw, &mut player_1), (player_2_raw, &mut player_2)] {
        **player = raw
            .lines()
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .filter(|s| &s[0..1] != "P")
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<VecDeque<i32>>();
    }

    println!("player 1: {:?}", player_1);
    println!("player 2: {:?}", player_2);

    while player_1.len() != 0 && player_2.len() != 0 {
        let c_1 = player_1.pop_front().unwrap();
        let c_2 = player_2.pop_front().unwrap();
        if c_1 > c_2 {
            player_1.push_back(c_1);
            player_1.push_back(c_2);
        } else {
            player_2.push_back(c_2);
            player_2.push_back(c_1);
        }

        println!("player 1: {:?}", player_1);
        println!("player 2: {:?}", player_2);
    }

    let winner = if player_1.len() != 0 {
        player_1
    } else {
        player_2
    };

    let mut res = 0;

    for (n, c) in winner.iter().enumerate() {
        res += (winner.len() as i32 - n as i32) * c;
    }

    println!("res: {}", res);

    Ok(())
}
