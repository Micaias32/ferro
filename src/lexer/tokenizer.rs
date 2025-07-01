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
            c if c.is_whitespace() => self.whitespace(),
            c if c.is_alphabetic() => self.ident(),
            c if c.is_ascii_digit() => self.number(),
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

    fn whitespace(&mut self) -> TokenKind {
        loop {
            let c = self.src[self.position];

            if c.is_whitespace() {
                self.position += 1;
            } else {
                self.position -= 1;
                return TokenKind::Whitespace;
            }
        }
    }

    fn ident(&mut self) -> TokenKind {
        // All call sites should only call if the character right now is alphabetic.
        // This is here only for sanity reasons
        debug_assert!(self.src[self.position].is_alphabetic());

        loop {
            let c = self.src[self.position];

            if c.is_alphabetic() || c.is_ascii_digit() {
                self.position += 1;
            } else {
                self.position -= 1;
                return TokenKind::Ident;
            }
        }
    }

    fn number(&mut self) -> TokenKind {
        loop {
            let c = self.src[self.position];

            if c.is_ascii_digit() {
                self.position += 1;
            } else {
                self.position -= 1;
                return TokenKind::Literal {
                    kind: super::LiteralKind::NumberLiteral,
                };
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::lexer::LiteralKind;

    use super::*;
    #[test]
    fn simple_tokenizing() {
        let code = "a <- 1 + 1;";
        let mut tokenizer = Tokenizer::new(code);

        let tokens = tokenizer.tokenize();

        assert_eq!(
            tokens,
            vec![
                Token::new(TokenKind::Ident, 1),
                Token::new(TokenKind::Whitespace, 1),
                Token::new(TokenKind::LessThan, 1),
                Token::new(TokenKind::Minus, 1),
                Token::new(TokenKind::Whitespace, 1),
                Token::new(
                    TokenKind::Literal {
                        kind: LiteralKind::NumberLiteral
                    },
                    1
                ),
                Token::new(TokenKind::Whitespace, 1),
                Token::new(TokenKind::Plus, 1),
                Token::new(TokenKind::Whitespace, 1),
                Token::new(
                    TokenKind::Literal {
                        kind: LiteralKind::NumberLiteral
                    },
                    1
                ),
                Token::new(TokenKind::Semi, 1),
                Token::new(TokenKind::Eof, 0)
            ]
        );
    }
}
