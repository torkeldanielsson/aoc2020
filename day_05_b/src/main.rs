use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let mut seats = vec![0; 891];

    for line in input.lines() {
        let chars: Vec<char> = line.chars().collect();
        let mut row: i32 = 0;
        for i in 0..7 {
            if chars[i] == 'B' {
                row |= 1 << (6 - i);
            }
        }
        let mut column: i32 = 0;
        for i in 0..3 {
            if chars[7 + i] == 'R' {
                column |= 1 << (2 - i);
            }
        }
        let seat_id = row * 8 + column;
        seats[seat_id as usize] = 1;
    }

    for i in 0..seats.len() {
        if seats[i] == 0 {
            println!("not taken: {}", i);
        }
    }

    Ok(())
}
