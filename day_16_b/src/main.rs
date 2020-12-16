use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let sections: Vec<&str> = input.split("\n\n").collect();
    let fields_raw = sections[0];
    let ticket_raw = sections[1];
    let other_tickets_raw = sections[2];

    let mut ticket_values: Vec<Vec<i32>> = Vec::new();

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

        let mut ticket_is_ok = true;

        for other_ticket_val in &other_ticket_values {
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

                    if other_ticket_val >= &range_min && other_ticket_val <= &range_max {
                        is_in_range = true;
                    }
                }
            }

            if !is_in_range {
                ticket_is_ok = false;
            }
        }

        if ticket_is_ok {
            for (i, v) in other_ticket_values.iter().enumerate() {
                if ticket_values.len() < i + 1 {
                    ticket_values.push(Vec::new());
                }
                ticket_values[i].push(v.to_owned());
            }
        }
    }

    // println!("{:?}", ticket_values);

    let mut field_name_to_pos: Vec<(String, Vec<i32>)> = Vec::new();

    for fields in fields_raw
        .lines()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
    {
        let field_parts: Vec<&str> = fields.split(": ").collect();
        let field = field_parts[0];

        print!("\nfield {}, matches: ", &field);

        let mut field_name_to_pos_entry: (String, Vec<i32>) = (field.to_string(), Vec::new());

        for (i, values) in ticket_values.iter().enumerate() {
            let mut values_match_field_rules = true;

            for value in values {
                let mut is_in_range = false;

                for field_ranges in field_parts[1].split(" or ") {
                    let range_vals: Vec<i32> = field_ranges
                        .split("-")
                        .map(|s| s.parse::<i32>().unwrap())
                        .collect();
                    let range_min = range_vals[0];
                    let range_max = range_vals[1];

                    if value >= &range_min && value <= &range_max {
                        is_in_range = true;
                    }
                }

                if !is_in_range {
                    values_match_field_rules = false;
                }
            }

            if values_match_field_rules {
                print!(" {}, ", i);
                field_name_to_pos_entry.1.push(i as i32);
            }
        }

        field_name_to_pos.push(field_name_to_pos_entry);
    }

    let mut still_some_left = true;

    while still_some_left {
        let mut values_to_remove = Vec::new();

        for field_name_to_pos_entry in &field_name_to_pos {
            if field_name_to_pos_entry.1.len() == 1 {
                values_to_remove.push(field_name_to_pos_entry.1[0]);
            }
        }

        for field_name_to_pos_entry in &mut field_name_to_pos {
            if field_name_to_pos_entry.1.len() > 1 {
                for n in &values_to_remove {
                    field_name_to_pos_entry.1.retain(|&x| x != *n);
                }
            }
        }

        still_some_left = false;
        for field_name_to_pos_entry in &field_name_to_pos {
            if field_name_to_pos_entry.1.len() > 1 {
                still_some_left = true;
            }
        }
    }

    let ticket_values: Vec<i32> = ticket_raw
        .lines()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .filter(|s| &s[0..2] != "yo")
        .nth(0)
        .unwrap()
        .split(",")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    println!("\n");

    let mut res: i64 = 1;

    for field_name_to_pos_entry in &field_name_to_pos {
        if &field_name_to_pos_entry.0[0..3] == "dep" {
            println!("{:?}", field_name_to_pos_entry);
            res *= ticket_values[field_name_to_pos_entry.1[0] as usize] as i64;
        }
    }

    println!("res: {}", res);

    // too low: 1003200

    Ok(())
}
