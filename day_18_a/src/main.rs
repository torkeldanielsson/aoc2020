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
                        let mut is_operator = true;
                        while is_operator {
                            if let Some(operator) = operator_stack.pop() {
                                match operator {
                                    Token::Operator { o: Operator::Add }
                                    | Token::Operator {
                                        o: Operator::Multiply,
                                    } => {
                                        output_queue.push(operator.to_owned());
                                    }
                                    _ => {
                                        is_operator = false;
                                    }
                                }
                                if !is_operator {
                                    operator_stack.push(operator);
                                }
                            } else {
                                is_operator = false;
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

        for operator in operator_stack {
            output_queue.push(operator);
        }

        let mut stack: Vec<i64> = Vec::new();

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

        //        println!("res: {}", stack.pop().unwrap());

        sum += stack.pop().unwrap();
    }

    println!("sum: {}", sum);

    Ok(())
}
