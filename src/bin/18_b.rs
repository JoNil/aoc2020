use aoc2020::get_input;

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

#[derive(Debug)]
pub enum Node {
    Add(Box<Node>, Box<Node>),
    Sub(Box<Node>, Box<Node>),
    Mul(Box<Node>, Box<Node>),
    Div(Box<Node>, Box<Node>),
    Num(i64),
}

fn is_op(rest: &[Token], op: &str) -> bool {
    if let [Token::Punct(s), ..] = rest {
        *s == op
    } else {
        false
    }
}

type NodeAndRest<'a> = (Box<Node>, &'a [Token<'a>]);

fn new_node(new_node: impl FnOnce(Box<Node>) -> Node, nr: NodeAndRest) -> NodeAndRest {
    (Box::new(new_node(nr.0)), nr.1)
}

fn expr<'a>(rest: &'a [Token]) -> NodeAndRest<'a> {
    mul(rest)
}

fn mul<'a>(rest: &'a [Token]) -> NodeAndRest<'a> {
    let (mut node, mut rest) = add(rest);

    loop {
        let nr = match rest {
            [Token::Punct("*"), ..] => new_node(move |rhs| Node::Mul(node, rhs), add(&rest[1..])),
            [Token::Punct("/"), ..] => new_node(move |rhs| Node::Div(node, rhs), add(&rest[1..])),
            _ => {
                return (node, rest);
            }
        };

        node = nr.0;
        rest = nr.1;
    }
}

fn add<'a>(rest: &'a [Token]) -> NodeAndRest<'a> {
    let (mut node, mut rest) = primary(rest);

    loop {
        let nr = match rest {
            [Token::Punct("+"), ..] => {
                new_node(move |rhs| Node::Add(node, rhs), primary(&rest[1..]))
            }
            [Token::Punct("-"), ..] => {
                new_node(move |rhs| Node::Sub(node, rhs), primary(&rest[1..]))
            }
            _ => {
                return (node, rest);
            }
        };

        node = nr.0;
        rest = nr.1;
    }
}

fn primary<'a>(rest: &'a [Token]) -> NodeAndRest<'a> {
    if is_op(rest, "(") {
        let (node, rest) = expr(&rest[1..]);

        if !is_op(rest, ")") {
            panic!("Expected: )");
        }

        return (node, &rest[1..]);
    }

    if let Token::Num(num) = rest[0] {
        return (Box::new(Node::Num(num as i64)), &rest[1..]);
    }

    panic!("expected an expression");
}

pub fn parse(rest: &[Token]) -> Box<Node> {
    let (node, rest) = expr(rest);

    if !rest.is_empty() {
        panic!("Extra token");
    }

    node
}

fn eval(node: &Node) -> i64 {
    match node {
        Node::Add(lhs, rhs) => eval(lhs) + eval(rhs),
        Node::Sub(lhs, rhs) => eval(lhs) - eval(rhs),
        Node::Mul(lhs, rhs) => eval(lhs) * eval(rhs),
        Node::Div(lhs, rhs) => eval(lhs) / eval(rhs),
        Node::Num(value) => *value,
    }
}

fn eval_line(input: &str) -> i64 {
    let tokens = tokenize(input);
    let node = parse(&tokens);
    eval(&node)
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
            693891
        );
    }

    #[test]
    fn test_eval_line() {
        assert_eq!(super::eval_line("1 + (2 * 3) + (4 * (5 + 6))"), 51);
        assert_eq!(super::eval_line("2 * 3 + (4 * 5)"), 46);
        assert_eq!(super::eval_line("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 1445);
        assert_eq!(
            super::eval_line("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"),
            669060
        );
        assert_eq!(
            super::eval_line("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
            23340
        );
    }
}
