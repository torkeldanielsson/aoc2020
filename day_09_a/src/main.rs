use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let mut numbers = vec![];

    for line in input.lines() {
        let number = line.parse::<i64>()?;

        if numbers.len() == 25 {
            let mut ok = false;
            'outer: for i in 0..24 {
                for j in i + 1..25 {
                    if numbers[i] + numbers[j] == number {
                        ok = true;
                        break 'outer;
                    }
                }
            }
            if !ok {
                println!("not ok: {}", number);
            }
        }

        numbers.push(number);
        if numbers.len() > 25 {
            numbers.drain(0..1);
        }
    }

    Ok(())
}
