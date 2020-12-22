use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    // mxmxvkd kfcds sqjhc nhms (contains dairy, fish)

    let mut knowledge: HashMap<String, HashSet<String>> = HashMap::new();

    let mut allergen_free: HashSet<String> = HashSet::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split(" (contains").collect();
        assert!(parts.len() == 2);
        let ingredients: Vec<String> = parts[0]
            .split(" ")
            .map(|s| s.trim())
            .map(|s| s.to_string())
            .collect();
        let allergens: Vec<String> = parts[1]
            .replace(")", "")
            .split(",")
            .map(|s| s.trim())
            .map(|s| s.to_string())
            .collect();

        let mut ingredient_set: HashSet<String> = HashSet::new();
        for ingredient in ingredients {
            ingredient_set.insert(ingredient.clone());
            allergen_free.insert(ingredient);
        }

        for allergen in allergens {
            if knowledge.contains_key(&allergen) {
                let intersection = knowledge[&allergen]
                    .intersection(&ingredient_set)
                    .map(|s| s.to_owned())
                    .collect();
                knowledge.remove(&allergen);
                knowledge.insert(allergen, intersection);
            } else {
                knowledge.insert(allergen, ingredient_set.clone());
            }
        }
    }

    {
        let mut processed_ingredients: HashSet<String> = HashSet::new();
        loop {
            let mut added_processed_ingredients = false;

            let mut ingredients_to_process: HashSet<String> = HashSet::new();

            for knol in &knowledge {
                if knol.1.len() == 1 {
                    let ingredient_to_maybe_process = knol.1.iter().nth(0).unwrap();
                    if !processed_ingredients.contains(ingredient_to_maybe_process) {
                        ingredients_to_process.insert(ingredient_to_maybe_process.to_owned());
                    }
                }
            }

            for ingredient in &ingredients_to_process {
                for knol in &mut knowledge {
                    if knol.1.len() != 1 {
                        knol.1.remove(ingredient);
                        added_processed_ingredients = true;
                    }
                }
            }

            processed_ingredients = processed_ingredients
                .union(&ingredients_to_process)
                .map(|s| s.to_owned())
                .collect();

            if !added_processed_ingredients {
                break;
            }
        }
    }

    for knol in &knowledge {
        assert!(knol.1.len() == 1);
        allergen_free.remove(knol.1.iter().nth(0).unwrap());
    }

    println!("{:?}", knowledge);

    let mut res = 0;

    for line in input.lines() {
        let parts: Vec<&str> = line.split(" (contains").collect();
        let ingredients: Vec<&str> = parts[0].split(" ").collect();

        for ingredient in ingredients {
            if allergen_free.contains(ingredient) {
                res += 1;
            }
        }
    }

    println!("{:?}", res);

    Ok(())
}
