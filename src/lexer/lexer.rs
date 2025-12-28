use std::iter::Peekable;
use std::str::Chars;
use crate::lexer::Token;

pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
}



impl<'a> Lexer<'a> {
    pub fn new(chars: &'a str) -> Self {
        Lexer { chars: chars.chars().peekable() }
    }

    fn skip_whitespace(&mut self) {
        while self.chars.next_if(|c| c.is_whitespace()).is_some() {}
    }

    fn parse_nontrivial_operator(&mut self) -> Option<Token> {
        match self.chars.next()? {
            '-' => self.chars.next()?.eq(&'>').then_some(Token::Arrow),
            '~' => self.chars.next()?.eq(&'>').then_some(Token::TildeArrow),
            '=' => self.chars.next()?.eq(&'>').then_some(Token::WideArrow),
            ':' => self.chars.next()?.eq(&'=').then_some(Token::Assign),
            '[' => match self.chars.next_if(|c| *c == '[') {
                Some(_) => Some(Token::LWideBracket),
                None => Some(Token::LBracket),
            },
            ']' => match self.chars.next_if(|c| *c == ']') {
                Some(_) => Some(Token::RWideBracket),
                None => Some(Token::RBracket),
            },
            _ => None,
        }
    }

    fn parse_number(&mut self) -> Option<f32> {
        let mut number = String::new();

        while let Some(c) = self.chars.next_if(|c| c.is_digit(10) || *c == '.') {
            number.push(c);
        }

        match number.parse::<f32>() {
            Ok(n) => Some(n),
            Err(_) => None,
        }
    }

    fn parse_value_pair(&mut self) -> Option<Token> {
        let first = self.parse_number()?;
        self.skip_whitespace();
        let second = self.parse_number()?;
        Some(Token::ValuePair(first, second))
    }

    fn parse_ident(&mut self) -> Option<Token> {
        match self.chars.next_if(|c| *c == '"') {
            Some(_) => todo!(),
            None => {
                let mut result = String::new();
                while let Some(c) = self.chars.next_if(|c| c.is_alphabetic() || *c == '_' || *c == '-') {
                    result.push(c);
                }
                Some(Token::Ident(result))
            },
        }
    }

    fn skip_comment(&mut self) {
        while let Some('#') = self.chars.peek() {
            while !matches!(self.chars.peek(), Some('\n')) {
                self.chars.next();
            }
            self.skip_whitespace();
        }
    }
}

impl Iterator for Lexer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();
        self.skip_comment();

        match self.chars.peek()? {
            '~' | '-' | '=' | ':' | '[' | ']' => return self.parse_nontrivial_operator(),
            '.' | '0' ..= '9' => return self.parse_value_pair(),
            c if c.is_alphabetic() || *c == '"' || *c == '_' => return self.parse_ident(),
            _ => {},
        }

        match self.chars.next()? {
            ',' => Some(Token::Comma),
            '+' => Some(Token::Plus),
            '(' => Some(Token::LParen),
            ')' => Some(Token::RParen),
            '{' => Some(Token::LCurly),
            '}' => Some(Token::RCurly),
            '|' => Some(Token::Pipe),
            '/' => Some(Token::Slash),
            '?' => Some(Token::QuestionMark),
            ';' => Some(Token::Semicolon),
            '§' => Some(Token::Section),
            '&' => Some(Token::Ampersand),
            _ => None,
        }
    }
}