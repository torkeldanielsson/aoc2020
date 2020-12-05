use std::error::Error;
use std::fs;

fn get_field(input: &str, field: &str) -> Option<String> {
    let pre_1: Vec<&str> = input
        .split(field)
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();

    if pre_1.len() == 1 || pre_1.len() == 2 {
        let pre_2;
        if pre_1.len() == 1 {
            pre_2 = pre_1[0];
        } else {
            pre_2 = pre_1[1];
        }

        let pre_3: Vec<&str> = pre_2
            .split(' ')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();

        if pre_3.len() >= 1 {
            return Some(pre_3[0].to_owned());
        }
    }

    return None;
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let passports: Vec<&str> = input
        .split("\n\n")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();

    let mut valid_count = 0;

    for passport_ref in passports {
        let mut passport = String::new();
        for c in passport_ref.chars() {
            if c == '\n' {
                passport.push(' ');
            } else {
                passport.push(c);
            }
        }

        let mut is_ok = true;

        if !(passport.contains("byr:")
            && passport.contains("iyr:")
            && passport.contains("eyr:")
            && passport.contains("hgt:")
            && passport.contains("hcl:#")
            && passport.contains("ecl:")
            && passport.contains("pid:"))
        {
            is_ok = false;
        } else {
            if let Some(byr) = get_field(&passport, "byr:") {
                if let Ok(byr_digits) = byr.parse::<i32>() {
                    if byr_digits < 1920 || byr_digits > 2002 {
                        is_ok = false;
                    }
                }
            } else {
                println!("can't get byr?!: {}", &passport);
                is_ok = false;
            }

            if let Some(iyr) = get_field(&passport, "iyr:") {
                if let Ok(iyr_digits) = iyr.parse::<i32>() {
                    if iyr_digits < 2010 || iyr_digits > 2020 {
                        is_ok = false;
                    }
                }
            } else {
                println!("can't get iyr?!: {}", &passport);
                is_ok = false;
            }

            if let Some(eyr) = get_field(&passport, "eyr:") {
                if let Ok(eyr_digits) = eyr.parse::<i32>() {
                    if eyr_digits < 2020 || eyr_digits > 2030 {
                        is_ok = false;
                    }
                }
            } else {
                println!("can't get eyr?!: {}", &passport);
                is_ok = false;
            }

            if let Some(hgt) = get_field(&passport, "hgt:") {
                if hgt.contains("cm") {
                    let parts: Vec<&str> = hgt
                        .split("cm")
                        .map(|s| s.trim())
                        .filter(|s| !s.is_empty())
                        .collect();
                    if parts.len() >= 1 {
                        if let Ok(val) = parts[0].parse::<i32>() {
                            if val < 150 || val > 193 {
                                is_ok = false;
                            }
                        }
                    }
                } else if hgt.contains("in") {
                    let parts: Vec<&str> = hgt
                        .split("in")
                        .map(|s| s.trim())
                        .filter(|s| !s.is_empty())
                        .collect();
                    if parts.len() >= 1 {
                        if let Ok(val) = parts[0].parse::<i32>() {
                            if val < 59 || val > 76 {
                                is_ok = false;
                            }
                        }
                    }
                } else {
                    println!("hgt invalid: {}", &hgt);
                    is_ok = false;
                }
            } else {
                println!("can't get hgt?!: {}", &passport);
                is_ok = false;
            }

            if let Some(hcl) = get_field(&passport, "hcl:#") {
                if hcl.len() != 6 {
                    is_ok = false;
                } else {
                    for c in hcl.chars() {
                        if !((c >= '0' && c <= '9') || (c >= 'a' && c <= 'f')) {
                            is_ok = false;
                            println!("invalid: {}", &hcl);
                        }
                    }
                }
            } else {
                println!("can't get hcl?!: {}", &passport);
                is_ok = false;
            }

            if let Some(ecl) = get_field(&passport, "ecl:") {
                if !(ecl == "amb"
                    || ecl == "blu"
                    || ecl == "brn"
                    || ecl == "gry"
                    || ecl == "grn"
                    || ecl == "hzl"
                    || ecl == "oth")
                {
                    is_ok = false;
                }
            } else {
                println!("can't get ecl?!: {}", &passport);
                is_ok = false;
            }

            if let Some(pid) = get_field(&passport, "pid:") {
                if pid.len() != 9 {
                    is_ok = false;
                } else {
                    for c in pid.chars() {
                        if !(c >= '0' && c <= '9') {
                            is_ok = false;
                            println!("invalid: {}", &pid);
                        }
                    }
                }
            } else {
                println!("can't get pid?!: {}", &passport);
                is_ok = false;
            }
        }

        if is_ok {
            valid_count += 1;

            //println!("{}", &passport);
        }
    }

    println!("valid passports: {}", valid_count);

    Ok(())
}
