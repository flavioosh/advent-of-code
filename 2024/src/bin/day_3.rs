use std::fs;
use std::iter::Peekable;
use std::mem::replace;
use std::str::Chars;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TokenType {
    Integer,

    LeftParen,
    RightParen,
    Comma,

    Mul,
    Do,
    Dont,

    Unknown,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Token {
    token_type: TokenType,
    lexeme: String,
}

struct Scanner<'a> {
    source: Peekable<Chars<'a>>,

    current_lexeme: String,
}

impl<'a> Scanner<'a> {
    fn new(source: &'a str) -> Scanner<'a> {
        let source = source.chars().peekable();
        Self {
            source,
            current_lexeme: String::with_capacity(16),
        }
    }

    fn peek(&mut self) -> Option<char> {
        self.source.peek().copied()
    }

    fn advance(&mut self) -> char {
        let char = self.source.next().expect("Unexpected end of input");
        self.current_lexeme.push(char);
        char
    }

    fn at_end(&mut self) -> bool {
        self.source.peek().is_none()
    }

    fn make_token(&mut self, token_type: TokenType) -> Token {
        let lexeme = replace(&mut self.current_lexeme, String::with_capacity(16));

        Token { token_type, lexeme }
    }

    fn number(&mut self) -> Token {
        while let Some(c) = self.peek() {
            if !c.is_ascii_digit() {
                break;
            }

            self.advance();
        }

        self.make_token(TokenType::Integer)
    }
}

impl Iterator for Scanner<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.at_end() {
            return None;
        }

        let c = self.advance();

        if c.is_ascii_digit() {
            return Some(self.number());
        }

        Some(match c {
            'm' => {
                if self.advance() != 'u' {
                    return Some(self.make_token(TokenType::Unknown));
                }

                if self.advance() != 'l' {
                    return Some(self.make_token(TokenType::Unknown));
                }

                return Some(self.make_token(TokenType::Mul));
            }

            'd' => {
                if self.advance() != 'o' {
                    return Some(self.make_token(TokenType::Unknown));
                }

                if let Some(c) = self.peek() {
                    if c != 'n' {
                        return Some(self.make_token(TokenType::Do));
                    }
                }
                self.advance();

                if self.advance() != '\'' {
                    return Some(self.make_token(TokenType::Unknown));
                }

                if self.advance() != 't' {
                    return Some(self.make_token(TokenType::Unknown));
                }

                return Some(self.make_token(TokenType::Dont));
            }

            '(' => self.make_token(TokenType::LeftParen),
            ')' => self.make_token(TokenType::RightParen),
            ',' => self.make_token(TokenType::Comma),

            _ => self.make_token(TokenType::Unknown),
        })
    }
}

#[derive(Debug)]
enum Statement {
    Mul(usize, usize),
    Do,
    Dont,
}

struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    const fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    fn at_end(&self) -> bool {
        self.current == self.tokens.len()
    }

    fn advance(&mut self) {
        if !self.at_end() {
            self.current += 1;
        }
    }

    fn current_token(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn current_token_type(&self) -> TokenType {
        self.tokens[self.current].token_type
    }

    fn consume(&mut self, token_type: TokenType) -> Option<()> {
        if self.current_token_type() == token_type {
            self.advance();
            return Some(());
        }

        None
    }

    fn consume_integer(&mut self) -> Option<usize> {
        if self.current_token_type() == TokenType::Integer {
            let integer = self
                .current_token()
                .lexeme
                .parse()
                .expect("Unexpected integer parsing error");
            self.advance();
            return Some(integer);
        }

        None
    }

    fn parse_mul(&mut self) -> Option<Statement> {
        self.consume(TokenType::Mul)?;
        self.consume(TokenType::LeftParen)?;
        let left_side = self.consume_integer()?;
        self.consume(TokenType::Comma)?;
        let right_side = self.consume_integer()?;
        self.consume(TokenType::RightParen)?;

        Some(Statement::Mul(left_side, right_side))
    }

    fn parse_do(&mut self) -> Option<Statement> {
        self.consume(TokenType::Do)?;
        self.consume(TokenType::LeftParen)?;
        self.consume(TokenType::RightParen)?;

        Some(Statement::Do)
    }

    fn parse_dont(&mut self) -> Option<Statement> {
        self.consume(TokenType::Dont)?;
        self.consume(TokenType::LeftParen)?;
        self.consume(TokenType::RightParen)?;

        Some(Statement::Dont)
    }

    fn run(&mut self) -> Vec<Statement> {
        let mut statements = Vec::new();

        while !self.at_end() {
            if let Some(statement) = self.parse_mul() {
                statements.push(statement);
                continue;
            }

            if let Some(statement) = self.parse_do() {
                statements.push(statement);
                continue;
            }

            if let Some(statement) = self.parse_dont() {
                statements.push(statement);
                continue;
            }

            self.advance();
        }

        statements
    }

    fn parse(tokens: Vec<Token>) -> Vec<Statement> {
        let mut parser = Self::new(tokens);
        parser.run()
    }
}

fn main() {
    let data = fs::read_to_string("res/day_3.txt").expect("Could not read file");

    let scanner = Scanner::new(&data);
    let statements = Parser::parse(scanner.collect());

    let total: usize = statements
        .iter()
        .map(|statement| match statement {
            Statement::Mul(left, right) => left * right,
            _ => 0,
        })
        .sum();

    println!("Part 1: {total}");

    let mut conditional_total = 0;
    let mut enabled = true;
    for statement in statements {
        match statement {
            Statement::Do => enabled = true,
            Statement::Dont => enabled = false,
            Statement::Mul(left, right) => {
                if enabled {
                    conditional_total += left * right;
                }
            }
        }
    }

    println!("Part 2: {conditional_total}");
}
