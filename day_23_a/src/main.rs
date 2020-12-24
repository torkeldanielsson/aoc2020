use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // let mut deck = vec![3, 8, 9, 1, 2, 5, 4, 6, 7];
    let mut deck = vec![3, 2, 6, 5, 1, 9, 4, 7, 8];

    for _ in 0..100 {
        println!("{:?}", deck);
        let current_cup = deck[0];
        let first_three = &deck[1..4];
        let mut destination = current_cup - 1;
        if destination == 0 {
            destination = 9;
        }
        while destination == first_three[0]
            || destination == first_three[1]
            || destination == first_three[2]
        {
            destination -= 1;
            if destination == 0 {
                destination = 9;
            }
        }
        let mut new_deck = Vec::new();
        let mut deck_i = 4;
        while deck_i < 9 {
            new_deck.push(deck[deck_i]);
            if deck[deck_i] == destination {
                new_deck.push(first_three[0]);
                new_deck.push(first_three[1]);
                new_deck.push(first_three[2]);
            }
            deck_i += 1;
        }
        new_deck.push(deck[0]);
        deck = new_deck;
    }

    println!("{:?}", deck);

    let mut i = 0;
    for j in 0..9 {
        if deck[j] == 1 {
            i = j + 1;
        }
    }
    for j in 0..8 {
        print!("{}", deck[(i + j) % 9]);
    }
    println!();

    Ok(())
}
