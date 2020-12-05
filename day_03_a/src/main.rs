use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let mut tree_count = 0;
    let mut x_pos = 0;

    for line in input.lines() {
        let chars: Vec<char> = line.chars().collect();
        if chars[x_pos % chars.len()] == '#' {
            tree_count += 1;
        }
        x_pos += 3;
    }

    println!("trees: {}", tree_count);

    Ok(())
}
