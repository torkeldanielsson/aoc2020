use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;
    let target: i64 = 373803594;
    let numbers: Vec<i64> = input.lines().map(|s| s.parse::<i64>().unwrap()).collect();

    for i in 0..numbers.len() {
        let mut sum: i64 = 0;
        let mut min: i64 = numbers[i];
        let mut max: i64 = numbers[i];

        for j in i..numbers.len() {
            sum += numbers[j];
            min = min.min(numbers[j]);
            max = max.max(numbers[j]);

            if sum == target {
                println!("{},{} => {}", min, max, min + max);
                return Ok(());
            }
            if sum > target {
                break; // Stop if we've exceeded the target
            }
        }
    }

    Ok(())
}
