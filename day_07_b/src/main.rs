use std::collections::HashMap;
use std::error::Error;
use std::fs;

struct BagWithCount {
    bag_color: String,
    count: i32,
}

fn find_inner_bags_count(start: &str, rules: &HashMap<&str, Vec<BagWithCount>>) -> i64 {
    let mut res = 1;

    for (input_bag, output_bags) in rules {
        if input_bag == &start {
            if output_bags.len() != 0 {
                for output_bag in output_bags {
                    res += output_bag.count as i64
                        * find_inner_bags_count(&output_bag.bag_color, rules);
                }
            }
        }
    }

    res
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let mut rules = HashMap::new();

    for line in input.lines().map(|s| s.trim()).filter(|s| !s.is_empty()) {
        let parts: Vec<&str> = line
            .split("bags contain")
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();

        assert!(parts.len() == 2);

        let input_bag_color = parts[0];

        if parts[1] != "no other bags." {
            let mut bags_produced = Vec::new();

            for output in parts[1]
                .split(", ")
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
            {
                let outputs: Vec<&str> = output
                    .split(" ")
                    .map(|s| s.trim())
                    .filter(|s| !s.is_empty())
                    .filter(|s| s != &"bag")
                    .filter(|s| s != &"bags")
                    .filter(|s| s != &"bags.")
                    .filter(|s| s != &"bag.")
                    .collect();
                let count = outputs[0].parse::<i32>()?;
                let bag_color = format!("{} {}", outputs[1], outputs[2]);

                // println!("{} -> {} {}", input_bag_color, count, bag_color);

                bags_produced.push(BagWithCount { bag_color, count });
            }

            rules.insert(input_bag_color, bags_produced);
        }
    }

    println!("{}", find_inner_bags_count("shiny gold", &rules) - 1);

    Ok(())
}
