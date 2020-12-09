use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let numbers: Vec<i64> = input.lines().map(|s| s.parse::<i64>().unwrap()).collect();

    for i in 0..numbers.len() {
        let mut sum: i64 = 0;
        let mut max: i64 = 0;
        let mut min: i64 = 999999999999999;
        for j in i..numbers.len() {
            sum += numbers[j];
            if numbers[j] < min {
                min = numbers[j];
            }
            if numbers[j] > max {
                max = numbers[j];
            }
            if sum == 1639024365 {
                println!("{},{} => {}", min, max, max + min);
            }
        }
    }

    Ok(())
}
