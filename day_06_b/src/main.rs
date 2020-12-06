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
        let mut line_sets = Vec::new();

        let lines: Vec<&str> = declaration
            .split("\n")
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();
        for line in lines {
            let mut unique_chars: HashSet<char> = HashSet::new();
            for c in line.chars() {
                if c >= 'a' && c <= 'z' {
                    unique_chars.insert(c);
                }
            }
            line_sets.push(unique_chars);
        }

        let mut res_set = line_sets[0].clone();

        for i in 1..line_sets.len() {
            res_set = res_set
                .intersection(&line_sets[i])
                .map(|c| c.to_owned())
                .collect();
        }

        // println!("{}\nunique: {}\n", declaration, res_set.len());

        res += res_set.len();
    }

    println!("res: {}", res);

    Ok(())
}
