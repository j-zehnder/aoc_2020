#[derive(Clone, Copy)]
enum PrecedenceMode {
    Equal,
    AddiMuli,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Op {
    Addi,
    Muli,
}

impl Op {
    fn from_char(c: char) -> Self {
        match c {
            '+' => Op::Addi,
            '*' => Op::Muli,
            _ => panic!("unknown op"),
        }
    }

    fn precedence(&self, mode: PrecedenceMode) -> usize {
        match mode {
            PrecedenceMode::Equal => 1,
            PrecedenceMode::AddiMuli => match self {
                Self::Addi => 2,
                Self::Muli => 1,
            },
        }
    }

    fn apply(&self, a: u64, b: u64) -> u64 {
        match self {
            Self::Addi => a + b,
            Self::Muli => a * b,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Token {
    Num(u64),
    Op(Op),
    LParen,
    RParen,
}

impl Token {
    fn from_char(c: char) -> Self {
        return if c.is_numeric() {
            Self::Num(c.to_digit(10).unwrap() as u64)
        } else if c == '(' {
            Self::LParen
        } else if c == ')' {
            Self::RParen
        } else {
            Self::Op(Op::from_char(c))
        };
    }
}

fn solve(equation: &str, mode: PrecedenceMode) -> u64 {
    let tokens = tokenize(equation);
    let rpn = infix_to_rpn(&tokens, mode);
    solve_rpn(&rpn)
}

fn tokenize(equation: &str) -> Vec<Token> {
    equation
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| Token::from_char(c))
        .collect::<Vec<Token>>()
}

fn infix_to_rpn(tokens: &[Token], mode: PrecedenceMode) -> Vec<Token> {
    let mut op_stack: Vec<Token> = Vec::new();
    let mut output: Vec<Token> = Vec::new();

    for token in tokens {
        match token {
            Token::Num(_n) => output.push(*token),
            Token::LParen => op_stack.push(*token),
            Token::RParen => {
                while *op_stack.last().unwrap() != Token::LParen {
                    output.push(op_stack.pop().unwrap());
                }
                op_stack.pop(); // discard the left paren
            }
            Token::Op(op) => {
                if op_stack.is_empty() {
                    op_stack.push(*token);
                } else {
                    loop {
                        if op_stack.is_empty() {
                            break;
                        }

                        if let Token::Op(top_op) = op_stack.last().unwrap() {
                            if top_op.precedence(mode) >= op.precedence(mode) {
                                output.push(op_stack.pop().unwrap())
                            } else {
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                    op_stack.push(*token);
                }
            }
        }
    }

    while let Some(token) = op_stack.pop() {
        output.push(token);
    }

    output
}

fn solve_rpn(tokens: &[Token]) -> u64 {
    let mut stack: Vec<u64> = Vec::new();

    for token in tokens {
        match token {
            Token::Num(n) => stack.push(*n),
            Token::Op(op) => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(op.apply(a, b));
            }
            _ => panic!("invalid RPL"),
        }
    }

    stack[0]
}

#[aoc_generator(day18)]
pub fn parse_input(input: &str) -> Vec<String> {
    input.lines().map(|s| s.to_string()).collect()
}

#[aoc(day18, part1)]
pub fn part1(equations: &[String]) -> u64 {
    equations
        .iter()
        .map(|e| solve(e, PrecedenceMode::Equal))
        .sum()
}

#[aoc(day18, part2)]
pub fn part2(equations: &[String]) -> u64 {
    equations
        .iter()
        .map(|e| solve(e, PrecedenceMode::AddiMuli))
        .sum()
}
