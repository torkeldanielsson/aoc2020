use std::fs;
use std::io;

fn mod_inverse(a: i64, m: i64) -> i64 {
    let a = a % m;
    for x in 1..m {
        if (a * x) % m == 1 {
            return x;
        }
    }
    1
}

fn chinese_remainder(residues: &[(i64, i64)]) -> i64 {
    let product: i64 = residues.iter().map(|&(_, modulus)| modulus).product();

    let mut sum = 0;
    for &(residue, modulus) in residues {
        let p = product / modulus;
        sum += residue * mod_inverse(p, modulus) * p;
    }

    ((sum % product) + product) % product
}

fn main() -> io::Result<()> {
    let input = fs::read_to_string("input")?;
    let lines: Vec<&str> = input.lines().collect();

    let bus_schedule = lines[1];

    let constraints: Vec<(i64, i64)> = bus_schedule
        .split(',')
        .enumerate()
        .filter(|(_, x)| *x != "x")
        .map(|(i, x)| {
            let bus_id = x.parse::<i64>().unwrap();
            let offset = -(i as i64);
            (offset.rem_euclid(bus_id), bus_id)
        })
        .collect();

    let result = chinese_remainder(&constraints);
    println!("res: {}", result);

    Ok(())
}
