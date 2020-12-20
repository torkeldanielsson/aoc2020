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

fn match_rules(
    message: &[char],
    message_i: usize,
    rule_i: i32,
    rules: &HashMap<i32, Vec<Vec<Token>>>,
) -> (bool, usize) {
    if message_i >= message.len() {
        return (true, 0);
    }

    let mc = match message[message_i] {
        'a' => Character::A,
        'b' => Character::B,
        _ => panic!(),
    };

    match rules[&rule_i].first().unwrap().first().unwrap() {
        Token::Character { c } => {
            if &mc == c {
                return (true, 1);
            } else {
                return (false, 0);
            }
        }
        _ => {}
    }

    for tokens in &rules[&rule_i] {
        let mut is_ok = true;
        let mut tmp_message_i = message_i;
        for token in tokens {
            match token {
                Token::Number { n } => {
                    let (next_res_ok, consumed) = match_rules(message, tmp_message_i, *n, rules);
                    if !next_res_ok {
                        is_ok = false;
                        break;
                    }
                    tmp_message_i += consumed;
                }
                _ => panic!("{:?}", token),
            }
        }
        if is_ok {
            println!("rule {} ({:?})", rule_i, tokens);
            return (true, tmp_message_i - message_i);
        }
    }

    (false, 0)
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input_test")?;

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

    //8: 42 | 42 8
    //11: 42 31 | 42 11 31

    rules.insert(
        8,
        vec![
            vec![Token::Number { n: 42 }],
            vec![Token::Number { n: 42 }, Token::Number { n: 8 }],
        ],
    );
    rules.insert(
        11,
        vec![
            vec![Token::Number { n: 42 }, Token::Number { n: 31 }],
            vec![
                Token::Number { n: 42 },
                Token::Number { n: 11 },
                Token::Number { n: 31 },
            ],
        ],
    );

    let mut res = 0;

    for message in messages.lines() {
        let tmp_chars: Vec<char> = message.chars().collect();
        let (match_res, match_len) = match_rules(&tmp_chars, 0, 0, &rules);
        if match_res && match_len == message.len() {
            println!("ok: {}", message);
            res += 1;
        }
    }

    println!("res: {}", res);

    Ok(())
}
