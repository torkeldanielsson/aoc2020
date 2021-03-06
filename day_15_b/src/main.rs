use std::collections::HashMap;

fn main() {
    let mut numbers: HashMap<i32, i32> = HashMap::new();

    // 9,6,0,10,18,2,1
    numbers.insert(9, 0);
    numbers.insert(6, 1);
    numbers.insert(0, 2);
    numbers.insert(10, 3);
    numbers.insert(18, 4);
    numbers.insert(2, 5);
    let mut next_number = 1;

    for turn in 6..30000000 {
        let new_next_number;
        if numbers.contains_key(&next_number) {
            new_next_number = turn - numbers[&next_number];
        } else {
            new_next_number = 0;
        }

        if turn == 30000000 - 1 {
            println!("{}", next_number);
        }

        numbers.insert(next_number, turn);
        next_number = new_next_number;
    }
}
