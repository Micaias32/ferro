use super::{Token, TokenKind};

pub struct Tokenizer {
    src: Vec<char>,
    position: usize,
}

impl Tokenizer {
    pub fn new(src: &'_ str) -> Self {
        let src = src.chars().collect();
        Self { src, position: 0 }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while let Some(token) = self.next_token() {
            tokens.push(token);
        }

        tokens
    }

    fn next_token(&mut self) -> Option<Token> {
        if self.src.len() == self.position {
            self.position += 1;
            return Some(Token::new(TokenKind::Eof, 0));
        }

        let char = self.src.get(self.position)?;
        let start = self.position;

        use super::TokenKind as T;
        let tok = match char {
            c if c.is_whitespace() => self.whitespace()?,
            ';' => T::Semi,
            ',' => T::Comma,
            '.' => T::Dot,
            '(' => T::OpenParen,
            ')' => T::CloseParen,
            '{' => T::OpenBrace,
            '}' => T::CloseBrace,
            '[' => T::OpenBracket,
            ']' => T::CloseBracket,
            '@' => T::At,
            '#' => T::Pound,
            '~' => T::Tilde,
            '?' => T::Question,
            ':' => T::Colon,
            '$' => T::Dollar,
            '=' => T::Eq,
            '!' => T::Bang,
            '<' => T::LessThan,
            '>' => T::GreaterThan,
            '-' => T::Minus,
            '&' => T::And,
            '|' => T::Or,
            '+' => T::Plus,
            '*' => T::Star,
            '/' => T::Slash,
            '^' => T::Caret,
            '%' => T::Percent,
            _ => unimplemented!(),
        };
        self.position += 1;
        Some(Token::new(tok, self.position - start))
    }
}
