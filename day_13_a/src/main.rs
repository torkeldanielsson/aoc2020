fn main() {
    // let start_time= 939;
    // let buses = vec![7, 13, 59, 31, 19];

    let start_time = 1001171;
    let buses = vec![17, 41, 37, 367, 19, 23, 29, 613, 13];

    let mut n = start_time;

    'outer: loop {
        for bus in &buses {
            if n % bus == 0 {
                println!("{}", (n - start_time) * bus);
                break 'outer;
            }
        }
        n += 1;
    }
}
