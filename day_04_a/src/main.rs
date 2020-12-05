use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let passports: Vec<&str> = input
        .split("\n\n")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();

    let mut valid_count = 0;

    for passport in passports {
        if passport.contains("byr:")
            && passport.contains("iyr:")
            && passport.contains("eyr:")
            && passport.contains("hgt:")
            && passport.contains("hcl:")
            && passport.contains("ecl:")
            && passport.contains("pid:")
        {
            valid_count += 1;
        }
    }

    println!("valid passports: {}", valid_count);

    Ok(())
}
