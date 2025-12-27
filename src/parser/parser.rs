use std::iter::Peekable;
use crate::lexer::{Lexer, Token};
use crate::parser::character::{Annotations, Character};
use crate::parser::SyntaxTree;

pub struct Parser<'a> {
    lexer: Peekable<Lexer<'a>>,
}

macro_rules! expect {
    ($iterator:expr, $pattern:pat, $then:expr) => {
        //#[must_use]
        match $iterator.next() {
            Some($pattern) => Some($then),
            _ => None,
        }
    };
    ($iterator:expr, $pattern:pat) => {
        expect!($iterator, $pattern, ())
    };
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Peekable<Lexer<'a>>) -> Self {
        Parser { lexer }
    }

    fn parse_arrow(&mut self) -> Option<SyntaxTree> {
        let Some(Token::ValuePair(x, y)) = self.lexer.next() else {
            return None;
        };
        match self.lexer.peek() {
            Some(Token::Arrow) => self.parse_line((x, y)),
            Some(Token::TildeArrow) => self.parse_line((x, y)),
            Some(Token::WideArrow) => self.parse_bound((x, y)),
            _ => None,
        }
    }

    fn parse_line(&mut self, start: (f32, f32)) -> Option<SyntaxTree> {
        let mut lines = vec![vec![start]];
        loop {
            match self.lexer.peek()? {
                Token::Arrow => {
                    let _ = self.lexer.next();
                    let Some(Token::ValuePair(x, y)) = self.lexer.next() else { return None; };
                    lines.last_mut().unwrap().push((x, y));
                    lines.push(vec![(x, y)]);
                },
                Token::TildeArrow => {
                    let _ = self.lexer.next();
                    let Some(Token::ValuePair(x, y)) = self.lexer.next() else { return None; };
                    lines.last_mut().unwrap().push((x, y));
                },
                _ => break,
            }
        }
        let _ = lines.pop();
        Some(SyntaxTree::Lines(lines))
    }

    fn parse_bound(&mut self, start: (f32, f32)) -> Option<SyntaxTree> {
        let Some(Token::WideArrow) = self.lexer.next() else { return None; };
        let Some(Token::ValuePair(x, y)) = self.lexer.next() else { return None; };
        Some(SyntaxTree::Bounds(start, (x, y)))
    }

    fn parse_value(&mut self) -> Option<SyntaxTree> {
        match self.lexer.peek()? {
            Token::LParen => {
                let _ = self.lexer.next()?;
                let tree = self.parse_tree()?;
                let Token::RParen = self.lexer.next()? else { return None; };
                Some(tree)
            },
            Token::LBracket => {
                let _ = self.lexer.next()?;
                let tree = self.parse_tree()?;
                let Token::RBracket = self.lexer.next()? else { return None; };
                Some(SyntaxTree::HalfwaySquare(Box::new(tree)))
            },
            Token::LWideBracket => {
                let _ = self.lexer.next()?;
                let tree = self.parse_tree()?;
                let Token::RWideBracket = self.lexer.next()? else { return None; };
                Some(SyntaxTree::Square(Box::new(tree)))
            },
            Token::Ident(_) => {
                let Some(Token::Ident(ident)) = self.lexer.next() else { return None; };
                Some(SyntaxTree::Ident(ident))
            }
            _ => self.parse_arrow()
        }
    }

    fn parse_inner(&mut self) -> Option<SyntaxTree> {
        let value = self.parse_value()?;
        if self.lexer.next_if_eq(&Token::LCurly).is_some() {
            let inner = self.parse_tree()?;
            match self.lexer.next() {
                Some(Token::RCurly) => Some(SyntaxTree::Inner(Box::new(value), Box::new(inner))),
                _ => None,
            }
        } else {
            Some(value)
        }
    }

    fn parse_shrink(&mut self) -> Option<SyntaxTree> {
        let mut shrinkage = 0;
        while self.lexer.next_if_eq(&Token::Ampersand).is_some() {
            shrinkage += 1;
        }
        Some(SyntaxTree::Shrink(shrinkage, Box::new(self.parse_inner()?)))
    }

    fn parse_vertical(&mut self) -> Option<SyntaxTree> {
        let mut result = vec![self.parse_shrink()?];
        while self.lexer.next_if_eq(&Token::Slash).is_some() {
            result.push(self.parse_shrink()?);
        }
        if result.len() == 1 {
            Some(result.pop()?)
        } else {
            Some(SyntaxTree::Vertical(result))
        }
    }

    fn parse_horizontal(&mut self) -> Option<SyntaxTree> {
        let mut result = vec![self.parse_vertical()?];
        while self.lexer.next_if_eq(&Token::Pipe).is_some() {
            result.push(self.parse_vertical()?);
        }
        if result.len() == 1 {
            Some(result.pop()?)
        } else {
            Some(SyntaxTree::Horizontal(result))
        }
    }

    fn parse_tree(&mut self) -> Option<SyntaxTree> {
        let mut result = vec![self.parse_horizontal()?];
        while self.lexer.next_if_eq(&Token::Plus).is_some() {
            result.push(self.parse_horizontal()?);
        }
        if result.len() == 1 {
            Some(result.pop()?)
        } else {
            Some(SyntaxTree::Plus(result))
        }
    }


    fn parse_bounds_as_rect_pair(&mut self) -> Option<((f32, f32), (f32, f32))> {
        let Some(Token::ValuePair(x, y)) = self.lexer.next() else {
            return None;
        };

        let Some(SyntaxTree::Bounds(start, end)) = self.parse_bound((x, y)) else {
            return None;
        };

        Some((start, end))
    }

    fn parse_annotation(&mut self, annotations: &mut Annotations) -> Option<()> {
        // := standalone
        // {} := outer + inner;
        match self.lexer.next()? {
            Token::LCurly => {
                let Some(Token::RCurly) = self.lexer.next() else {
                    return None;
                };

                let Some(Token::Assign) = self.lexer.next() else {
                    return None;
                };

                let start = self.parse_bounds_as_rect_pair()?;

                let Some(Token::Plus) = self.lexer.next() else {
                    return None;
                };

                let end = self.parse_bounds_as_rect_pair()?;

                annotations.inner = (start, end);

                Some(())
            },
            Token::Assign => {
                let rect = self.parse_bounds_as_rect_pair()?;

                annotations.standalone = rect;

                Some(())
            },
            _ => None,
        }
    }

    fn parse_annotations(&mut self) -> Option<Annotations> {
        let mut result = Annotations::new();
        while self.lexer.next_if_eq(&Token::QuestionMark).is_some() {
            self.parse_annotation(&mut result)?;
        }
        Some(result)
    }
}


impl Iterator for Parser<'_> {
    type Item = Character;

    fn next(&mut self) -> Option<Character> {
        //[§|E] name[, name]* := value [? [op|E] := value]* ;
        let is_radical = self.lexer.next_if(|it| matches!(it, Token::Section)).is_some();
        let mut names = vec![expect!(self.lexer, Token::Ident(n), n)?];
        loop {
            match self.lexer.next() {
                Some(Token::Comma) => {},
                Some(Token::Assign) => break,
                _ => return None,
            }
            names.push(expect!(self.lexer, Token::Ident(n), n)?);
        }
        let syntax_tree = self.parse_tree()?;

        let annotations = self.parse_annotations()?;

        let Some(Token::Semicolon) = self.lexer.next() else { return None; };

        Some(Character {
            is_radical,
            names,
            syntax_tree,
            annotations,
        })
    }
}