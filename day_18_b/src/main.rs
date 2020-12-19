use std::error::Error;
use std::fs;

#[derive(Clone, Debug, PartialEq)]
enum Operator {
    Add,
    Multiply,
}

#[derive(Clone, Debug, PartialEq)]
enum Token {
    Number { n: i64 },
    Operator { o: Operator },
    LeftParenthesis,
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let mut sum = 0;

    for line in input
        .lines()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.replace("(", " ( "))
        .map(|s| s.replace(")", " ) "))
    {
        let mut output_queue = Vec::new();
        let mut operator_stack = Vec::new();

        for token in line.split(" ").map(|s| s.trim()).filter(|s| !s.is_empty()) {
            match token.chars().nth(0).unwrap() {
                '1'..='9' => {
                    output_queue.push(Token::Number {
                        n: token.parse::<i64>().unwrap(),
                    });
                }
                '+' | '*' => {
                    {
                        let mut proceed = true;
                        while proceed {
                            if let Some(operator_from_stack) = operator_stack.pop() {
                                match operator_from_stack {
                                    Token::Operator { o: Operator::Add } => {
                                        output_queue.push(operator_from_stack.to_owned());
                                    }
                                    Token::Operator {
                                        o: Operator::Multiply,
                                    } => {
                                        if token == "*" {
                                            output_queue.push(operator_from_stack.to_owned());
                                        } else {
                                            proceed = false
                                        }
                                    }
                                    _ => {
                                        proceed = false;
                                    }
                                }
                                if !proceed {
                                    operator_stack.push(operator_from_stack);
                                }
                            } else {
                                proceed = false;
                            }
                        }
                    }

                    if token == "+" {
                        operator_stack.push(Token::Operator { o: Operator::Add });
                    }
                    if token == "*" {
                        operator_stack.push(Token::Operator {
                            o: Operator::Multiply,
                        });
                    }
                }
                '(' => {
                    operator_stack.push(Token::LeftParenthesis);
                }
                ')' => {
                    while !operator_stack.is_empty()
                        && operator_stack.last().unwrap() != &Token::LeftParenthesis
                    {
                        output_queue.push(operator_stack.pop().unwrap());
                    }
                    if !operator_stack.is_empty()
                        && operator_stack.last().unwrap() == &Token::LeftParenthesis
                    {
                        operator_stack.pop();
                    }
                }
                _ => panic!(),
            }
        }

        operator_stack.reverse();
        for operator in operator_stack {
            output_queue.push(operator);
        }

        let mut stack: Vec<i64> = Vec::new();

        // println!("{:?}", output_queue);

        for token in output_queue {
            match token {
                Token::Number { n } => {
                    stack.push(n);
                }
                Token::Operator { o } => match o {
                    Operator::Add => {
                        let res = stack.pop().unwrap() + stack.pop().unwrap();
                        stack.push(res);
                    }
                    Operator::Multiply => {
                        let res = stack.pop().unwrap() * stack.pop().unwrap();
                        stack.push(res);
                    }
                },
                _ => panic!(),
            }
        }

        //println!("res: {}", stack.last().unwrap());

        sum += stack.pop().unwrap();
    }

    println!("sum: {}", sum);

    Ok(())
}
