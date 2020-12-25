use std::collections::HashSet;
use std::error::Error;

fn calc_key(loop_count: i32, subject_number: i64) -> i64 {
    let mut v = 1;

    for _ in 0..loop_count {
        v *= subject_number;
        v = v % 20201227;
    }

    v
}

fn main() -> Result<(), Box<dyn Error>> {
    let subject_number = 7;

    let mut v = 1;

    let mut loop_counter = 1;

    // let pub_key_1 = 5764801;
    // let pub_key_2 = 17807724;
    let pub_key_1 = 1717001;
    let pub_key_2 = 523731;

    let mut ok = 0;

    let mut loop_1 = 0;
    let mut loop_2 = 0;

    loop {
        v *= subject_number;
        v = v % 20201227;

        if v == pub_key_1 {
            loop_1 = loop_counter;
            ok += 1;
        }
        if v == pub_key_2 {
            loop_2 = loop_counter;
            ok += 1;
        }

        if ok == 2 {
            break;
        }

        loop_counter += 1;
    }

    println!("1: {:?}", loop_1);
    println!("2: {:?}", loop_2);

    let key_1 = calc_key(loop_2, pub_key_1);
    let key_2 = calc_key(loop_1, pub_key_2);

    println!("key_1: {:?}", key_1);
    println!("key_2: {:?}", key_2);

    Ok(())
}
