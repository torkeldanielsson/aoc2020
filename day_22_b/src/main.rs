use std::collections::HashSet;
use std::collections::VecDeque;
use std::error::Error;
use std::fs;

#[derive(Clone, Debug, PartialEq)]
enum Player {
    Player1,
    Player2,
}

fn score(hand: &VecDeque<i32>) -> i32 {
    let mut res = 0;
    for (n, c) in hand.iter().enumerate() {
        res += (hand.len() as i32 - n as i32) * c;
    }
    return res;
}

fn play(
    game_counter: &mut i32,
    player_1_in: &VecDeque<i32>,
    player_2_in: &VecDeque<i32>,
) -> (Player, i32) {
    *game_counter += 1;
    //let game_id = game_counter.clone();
    //let mut round = 1;

    let mut player_1 = player_1_in.clone();
    let mut player_2 = player_2_in.clone();

    let mut earlier_states = HashSet::new();

    //println!("Game {:10}", game_id);

    while player_1.len() != 0 && player_2.len() != 0 {
        //round += 1;
        //println!("player 1: {:?}", player_1);
        //println!("player 2: {:?}", player_2);

        let state = (player_1.clone(), player_2.clone());
        if earlier_states.contains(&state) {
            //println!("player 1 win (1)");
            return (Player::Player1, score(&player_1));
        }
        earlier_states.insert(state);

        let c_1 = player_1.pop_front().unwrap();
        let c_2 = player_2.pop_front().unwrap();

        let mut got_winner = false;
        let mut sub_game_winner = Player::Player1;

        if c_1 > player_1.len() as i32 || c_2 > player_2.len() as i32 {
            if c_1 > c_2 {
                //println!("player 1 win (2)");
                sub_game_winner = Player::Player1;
                got_winner = true;
            } else {
                //println!("player 2 win (2)");
                sub_game_winner = Player::Player2;
                got_winner = true;
            }
        }

        if !got_winner {
            //println!("Playing a sub-game to determine the winner...");

            sub_game_winner = play(
                game_counter,
                &player_1.iter().map(|i| i.to_owned()).collect::<Vec<i32>>()[0..(c_1 as usize)]
                    .iter()
                    .map(|i| i.to_owned())
                    .collect(),
                &player_2.iter().map(|i| i.to_owned()).collect::<Vec<i32>>()[0..(c_2 as usize)]
                    .iter()
                    .map(|i| i.to_owned())
                    .collect(),
            )
            .0;
            /*
            match sub_game_winner {
                Player::Player1 => {
                    println!("player 1 win (3)");
                }
                Player::Player2 => {
                    println!("player 2 win (3)");
                }
            }*/
        }

        match sub_game_winner {
            Player::Player1 => {
                player_1.push_back(c_1);
                player_1.push_back(c_2);
            }
            Player::Player2 => {
                player_2.push_back(c_2);
                player_2.push_back(c_1);
            }
        }
    }

    if player_1.len() != 0 {
        return (Player::Player1, score(&player_1));
    } else {
        return (Player::Player2, score(&player_2));
    };
}

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

    let mut game_counter = 0;
    let winner = play(&mut game_counter, &player_1, &player_2);

    println!("winner: {:?}", winner);

    Ok(())
}
