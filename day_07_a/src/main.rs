use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::fs;

struct BagWithCount {
    bag_color: String,
}

fn find_outer_bags(inner: &str, rules: &HashMap<&str, Vec<BagWithCount>>) -> HashSet<String> {
    let mut res: HashSet<String> = HashSet::new();

    for (input_bag, output_bags) in rules {
        for output_bag in output_bags {
            if output_bag.bag_color == inner {
                res.insert(input_bag.to_owned().to_owned());

                let inner_res = find_outer_bags(input_bag.to_owned(), rules);
                res = res.union(&inner_res).map(|c| c.to_owned()).collect();
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
                let bag_color = format!("{} {}", outputs[1], outputs[2]);

                // println!("{} -> {} {}", input_bag_color, count, bag_color);

                bags_produced.push(BagWithCount { bag_color });
            }

            rules.insert(input_bag_color, bags_produced);
        }
    }

    println!("{:?}", find_outer_bags("shiny gold", &rules));
    println!("{}", find_outer_bags("shiny gold", &rules).len());

    Ok(())
}
