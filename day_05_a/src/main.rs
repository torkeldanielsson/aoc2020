use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let mut highest_seat = 0;

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
        if seat_id > highest_seat {
            highest_seat = seat_id;
        }
    }

    println!("res: {}", highest_seat);

    Ok(())
}
