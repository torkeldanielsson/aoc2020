use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // let mut deck_in = vec![3, 8, 9, 1, 2, 5, 4, 6, 7];
    let mut deck_in = vec![2, 1, 9, 3, 4, 7, 8, 6, 5];

    for i in 10..1000001 {
        deck_in.push(i);
    }

    for d in &mut deck_in {
        *d -= 1;
    }

    let mut deck = vec![0; deck_in.len()];

    for d in &deck_in {
        let current = deck_in[*d as usize];
        let next = deck_in[((d + 1) % deck_in.len()) as usize];
        deck[current] = next;
    }

    let mut current_cup = deck_in[0];

    for _ in 0..10000000 {
        let mov0 = deck[current_cup];
        let mov1 = deck[mov0];
        let mov2 = deck[mov1];
        let next_current_cup = deck[mov2];
        deck[current_cup] = next_current_cup;

        let mut target = current_cup;
        if target == 0 {
            target = deck.len();
        }
        target -= 1;
        while target == mov0 || target == mov1 || target == mov2 {
            if target == 0 {
                target = deck.len();
            }
            target -= 1;
        }

        deck[mov2] = deck[target];
        deck[target] = mov0;

        current_cup = next_current_cup;
    }

    let mut a = deck[0];
    let mut b = deck[a];
    a += 1;
    b += 1;
    println!("{}", a);
    println!("{}", b);
    println!("{}", a as i64 * b as i64);

    Ok(())
}
