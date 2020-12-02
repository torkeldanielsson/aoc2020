use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let mut valid = 0;

    for line in input.lines() {
        let parts1: Vec<&str> = line
            .split(':')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();
        assert!(parts1.len() == 2);

        let parts2: Vec<&str> = parts1[0]
            .split(' ')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();
        assert!(parts2.len() == 2);

        let parts3: Vec<&str> = parts2[0]
            .split('-')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();
        assert!(parts3.len() == 2);

        let min_range = parts3[0].parse::<i32>().unwrap();
        let max_range = parts3[1].parse::<i32>().unwrap();
        let character = parts2[1].chars().next().unwrap();
        let password = parts1[1];

        let mut count = 0;
        for c in password.chars() {
            if c == character {
                count += 1;
            }
        }

        if !(count < min_range || count > max_range) {
            valid += 1;
        }
    }

    println!("valid: {}", valid);

    Ok(())
}
