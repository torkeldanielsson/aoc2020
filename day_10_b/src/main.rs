use std::error::Error;
use std::fs;

fn count_possibilities(i: usize, numbers: &[i64]) -> i64 {
    if i == numbers.len() - 1 {
        return 1;
    }

    let mut res = 0;

    for j in 1..4 {
        if i + j < numbers.len() && numbers[i + j] - numbers[i] < 4 {
            res += count_possibilities(i + j, numbers);
        }
    }

    return res;
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let mut numbers: Vec<i64> = input.lines().map(|s| s.parse::<i64>().unwrap()).collect();

    numbers.push(0);

    numbers.sort();

    numbers.push(numbers[numbers.len() - 1] + 3);

    let mut combinations = vec![0; numbers.len()];

    for (i, n) in numbers.iter().enumerate() {
        for j in i + 1..i + 4 {
            if j < numbers.len() && numbers[j] - n < 4 {
                combinations[i] += 1;
            }
        }
    }

    let mut start = 0;

    let mut res = 1;

    for i in 2..combinations.len() {
        if i > start + 2
            && combinations[i] == 1
            && combinations[i - 1] == 1
            && combinations[i - 2] == 1
        {
            res *= count_possibilities(0, &numbers[start..i]);
            start = i;
        }
    }

    res *= count_possibilities(0, &numbers[start..combinations.len()]);

    println!("res: {}", res);

    Ok(())
}
