use aoc2020::get_input;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    Punct(&'a str),
    Num(i32),
}

fn get_number(p: &str) -> &str {
    p.split(|c: char| !c.is_ascii_digit()).next().unwrap()
}

fn read_punct(p: &str) -> usize {
    if p.chars().next().unwrap().is_ascii_punctuation() {
        1
    } else {
        0
    }
}

pub fn tokenize<'a>(input: &'a str) -> Vec<Token<'a>> {
    let mut tokens = Vec::new();
    let mut p = input;

    while !p.is_empty() {
        let next = p.chars().next().unwrap();

        if next.is_whitespace() {
            p = &p[next.len_utf8()..];
            continue;
        }

        if next.is_ascii_digit() {
            let number_str = get_number(p);
            tokens.push(Token::Num(number_str.parse::<i32>().unwrap()));
            p = &p[number_str.len()..];
            continue;
        }

        let punct_len = read_punct(p);
        if punct_len > 0 {
            tokens.push(Token::Punct(&p[..punct_len]));
            p = &p[punct_len..];
            continue;
        }
    }

    tokens
}

fn eval(tokens: &[Token]) -> i64 {
    let mut stack = Vec::new();
    let mut op_stack: Vec<&dyn Fn(i64, i64) -> i64> = Vec::new();

    let mut token_index = 0;

    while token_index < tokens.len() {
        match tokens[token_index] {
            Token::Num(value) => stack.push(value as i64),
            Token::Punct(punct) => match punct {
                "+" => op_stack.push(&Add::add),
                "-" => op_stack.push(&Sub::sub),
                "*" => op_stack.push(&Mul::mul),
                "/" => op_stack.push(&Div::div),
                "(" => {
                    let mut end_index = token_index;
                    let mut depth = 1;

                    for (index, token) in tokens[(token_index + 1)..].iter().enumerate() {
                        if *token == Token::Punct(")") {
                            depth -= 1;
                            if depth == 0 {
                                end_index = token_index + index + 1;
                                break;
                            }
                        }

                        if *token == Token::Punct("(") {
                            depth += 1;
                        }
                    }

                    stack.push(eval(&tokens[(token_index + 1)..end_index]));
                    token_index = end_index;
                }
                _ => panic!(),
            },
        }

        if stack.len() == 2 && op_stack.len() == 1 {
            let res = op_stack[0](stack[0], stack[1]);
            op_stack.remove(0);
            stack.remove(0);
            stack.remove(0);
            stack.push(res);
        }

        token_index += 1;
    }

    assert!(stack.len() == 1);
    stack[0]
}

fn eval_line(input: &str) -> i64 {
    let tokens = tokenize(input);
    eval(&tokens)
}

fn solve(input: &str) -> i64 {
    input.lines().map(eval_line).sum()
}

fn main() {
    let input = get_input();

    let result = solve(&input);

    println!("{:?}", result);
}

#[cfg(test)]
mod test {

    #[test]
    fn test_solve() {
        assert_eq!(
            super::solve(
                "2 * 3 + (4 * 5)
5 + (8 * 3 + 9 + 3 * 4 * 3)
5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))
((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"
            ),
            26335
        );
    }

    #[test]
    fn test_eval_line() {
        assert_eq!(super::eval_line("1 + 2 * 3 + 4 * 5 + 6"), 71);
        assert_eq!(super::eval_line("2 * 3 + (4 * 5)"), 26);
        assert_eq!(super::eval_line("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 437);
        assert_eq!(
            super::eval_line("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"),
            12240
        );
        assert_eq!(
            super::eval_line("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
            13632
        );
    }
}
