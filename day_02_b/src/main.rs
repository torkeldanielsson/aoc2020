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

        let a = parts3[0].parse::<usize>().unwrap() - 1;
        let b = parts3[1].parse::<usize>().unwrap() - 1;
        let character = parts2[1].chars().next().unwrap();
        let password = parts1[1].chars().collect::<Vec<char>>();

        if a < password.len() && b < password.len() {
            let a_ch = password[a];
            let b_ch = password[b];
            if a_ch != character && b_ch == character || a_ch == character && b_ch != character {
                valid += 1;
            }
        }
    }

    println!("valid: {}", valid);

    Ok(())
}
