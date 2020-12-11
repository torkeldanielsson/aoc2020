use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let mut numbers: Vec<i64> = input.lines().map(|s| s.parse::<i64>().unwrap()).collect();

    numbers.push(0);

    numbers.sort();

    let mut counts = vec![0; 4];

    counts[3] += 1;

    for i in 1..numbers.len() {
        let a = numbers[i - 1];
        let b = numbers[i];
        counts[(b - a) as usize] += 1;
        //println!("{}", b-a);
    }
    println!("{:?}", counts);

    println!("res: {}", counts[1] * counts[3]);

    Ok(())
}
