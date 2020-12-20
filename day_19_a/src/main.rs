use std::collections::HashMap;
use std::error::Error;
use std::fs;

#[derive(Clone, Debug, PartialEq)]
enum Character {
    A,
    B,
}

#[derive(Clone, Debug, PartialEq)]
enum Token {
    Number { n: i32 },
    Character { c: Character },
}

#[derive(Clone, Debug, PartialEq)]
enum RuleToken {
    Char { c: Character },
    Next { n: Vec<Vec<RuleToken>> },
}

fn parse_tokens(i: i32, rules: &HashMap<i32, Vec<Vec<Token>>>) -> RuleToken {
    match rules[&i].first().unwrap().first().unwrap() {
        Token::Character { c } => {
            return RuleToken::Char { c: c.clone() };
        }
        _ => {}
    }

    let mut next: Vec<Vec<RuleToken>> = Vec::new();

    for tokens in &rules[&i] {
        let mut next_list: Vec<RuleToken> = Vec::new();
        for token in tokens {
            match token {
                Token::Number { n } => {
                    next_list.push(parse_tokens(*n, rules));
                }
                _ => panic!(),
            }
        }
        next.push(next_list);
    }

    return RuleToken::Next { n: next };
}

fn match_rules(message: &[char], rule_token: &RuleToken) -> (bool, usize) {
    if message.len() == 0 {
        return (true, 0);
    }

    let mc = match message.first().unwrap() {
        'a' => Character::A,
        'b' => Character::B,
        _ => panic!(),
    };

    match rule_token {
        RuleToken::Char { c } => {
            if &mc == c {
                return (true, 1);
            }
        }
        RuleToken::Next { n } => {
            for next_list in n {
                let mut is_ok = true;
                let mut consumed_count = 0;
                for next in next_list {
                    let (next_res_ok, next_res_count) =
                        match_rules(&message[consumed_count..], next);
                    if !next_res_ok {
                        is_ok = false;
                        break;
                    }
                    consumed_count += next_res_count;
                }
                if is_ok {
                    return (true, consumed_count);
                }
            }
        }
    }

    (false, 0)
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let main_parts: Vec<&str> = input.split("\n\n").collect();
    let rules_raw: &str = main_parts[0];
    let messages: &str = main_parts[1];

    let mut rules: HashMap<i32, Vec<Vec<Token>>> = HashMap::new();

    for rule_line in rules_raw.lines() {
        let rule_line_parts: Vec<&str> = rule_line.split(": ").collect();
        let n = rule_line_parts[0].parse::<i32>().unwrap();
        let mut outputs: Vec<Vec<Token>> = Vec::new();
        for outputs_str in rule_line_parts[1].split("|") {
            let mut output: Vec<Token> = Vec::new();
            //println!("outputs_str: {:?}", outputs_str);
            for o in outputs_str
                .split(" ")
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
            {
                match o.chars().nth(0).unwrap() {
                    '1'..='9' => {
                        output.push(Token::Number {
                            n: o.parse::<i32>().unwrap(),
                        });
                    }
                    '"' => match o {
                        "\"a\"" => {
                            output.push(Token::Character { c: Character::A });
                        }
                        "\"b\"" => {
                            output.push(Token::Character { c: Character::B });
                        }
                        _ => panic!(),
                    },
                    _ => panic!(),
                }
            }
            outputs.push(output);
        }
        rules.insert(n, outputs);
    }

    // println!("{:?}", rules);

    let rule_tokens: RuleToken = parse_tokens(0, &rules);

    //println!("{:?}", rule_tokens);

    let mut res = 0;

    for message in messages.lines() {
        let tmp_chars: Vec<char> = message.chars().collect();
        let (match_res, match_len) = match_rules(&tmp_chars, &rule_tokens);
        if match_res && match_len == message.len() {
            res += 1;
        }
    }

    println!("res: {}", res);

    Ok(())
}
