use std::error::Error;
use std::fs;

fn count_trees(input: &str, right: i32, down: i32) -> i64 {
    let mut tree_count = 0;
    let mut x_pos = 0;

    for (y, line) in input.lines().enumerate() {
        let chars: Vec<char> = line.chars().collect();
        if (y as i32) % down == 0 {
            if chars[x_pos % chars.len()] == '#' {
                tree_count += 1;
            }
            x_pos += right as usize;
        }
    }

    tree_count
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    println!(
        "res: {}",
        count_trees(&input, 1, 1)
            * count_trees(&input, 3, 1)
            * count_trees(&input, 5, 1)
            * count_trees(&input, 7, 1)
            * count_trees(&input, 1, 2)
    );

    Ok(())
}
