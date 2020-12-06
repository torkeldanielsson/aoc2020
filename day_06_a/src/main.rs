use std::collections::HashSet;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let declarations: Vec<&str> = input
        .split("\n\n")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();

    let mut res = 0;

    for declaration in declarations {
        let mut unique_chars: HashSet<char> = HashSet::new();
        for c in declaration.chars() {
            if c >= 'a' && c <= 'z' {
                unique_chars.insert(c);
            }
        }

        // println!("{}\nunique: {}\n", declaration, unique_chars.len());

        res += unique_chars.len();
    }

    println!("res: {}", res);

    Ok(())
}
