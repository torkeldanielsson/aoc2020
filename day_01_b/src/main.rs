use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let mut numbers = vec![];

    for line in input.lines() {
        numbers.push(line.parse::<i32>()?);
    }

    for i in 0..numbers.len() {
        for j in i + 1..numbers.len() {
            for k in j + 1..numbers.len() {
                if numbers[i] + numbers[j] + numbers[k] == 2020 {
                    println!("{} + {} + {} = 2020!", numbers[i], numbers[j], numbers[k]);
                    println!(
                        "{} * {} * {} = {}",
                        numbers[i],
                        numbers[j],
                        numbers[k],
                        numbers[i] * numbers[j] * numbers[k]
                    );
                }
            }
        }
    }

    Ok(())
}
