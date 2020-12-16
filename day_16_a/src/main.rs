use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let sections: Vec<&str> = input.split("\n\n").collect();
    let fields_raw = sections[0];
    //let ticket_raw = sections[1];
    let other_tickets_raw = sections[2];

    let mut res = 0;

    for other_ticket in other_tickets_raw
        .lines()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .filter(|s| &s[0..2] != "ne")
    {
        let other_ticket_values: Vec<i32> = other_ticket
            .split(",")
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        for other_ticket_val in other_ticket_values {
            let mut is_in_range = false;

            for fields in fields_raw
                .lines()
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
            {
                let field_parts: Vec<&str> = fields.split(": ").collect();
                // let field = field_parts[0];

                for field_ranges in field_parts[1].split(" or ") {
                    let range_vals: Vec<i32> = field_ranges
                        .split("-")
                        .map(|s| s.parse::<i32>().unwrap())
                        .collect();
                    let range_min = range_vals[0];
                    let range_max = range_vals[1];

                    if other_ticket_val >= range_min && other_ticket_val <= range_max {
                        is_in_range = true;
                    }
                }
            }

            if !is_in_range {
                println!("not ok: {}", other_ticket_val);
                res += other_ticket_val;
            }
        }
    }

    println!("res: {}", res);

    Ok(())
}
