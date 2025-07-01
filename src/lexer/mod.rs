pub mod tokenizer;

/// A token is just its kind and its length
pub struct Token {
    kind: TokenKind,
    len: usize,
}

/// We have all kinds of tokens here
pub enum TokenKind {
    /// // comment?
    LineComment,

    /// /* block comment? */
    BlockComment,

    /// any whitespace
    Whitespace,

    /// any identifier
    Ident,

    /// Literals?
    Literal {
        kind: LiteralKind,
        suffix_start: u32,
    },

    /// ;
    Semi,

    /// ,
    Comma,

    /// .
    Dot,

    /// (
    OpenParen,

    /// )
    CloseParen,

    /// {
    OpenBrace,

    /// }
    CloseBrace,

    /// [
    OpenBracket,

    /// ]
    CloseBracket,

    /// @
    At,

    /// #
    Pound,

    /// ~
    Tilde,

    /// ?
    Question,

    /// :
    Colon,

    /// $
    Dollar,

    /// =
    Eq,

    /// !
    Bang,

    /// <
    LessThan,

    /// >
    GreaterThan,

    /// -
    Minus,

    /// &
    And,

    /// |
    Or,

    /// +
    Plus,

    /// *
    Star,

    /// /
    Slash,

    /// ^
    Caret,

    /// %
    Percent,

    /// End of input
    Eof,
}

pub enum LiteralKind {
    NumberLiteral,
}

impl Token {
    pub fn new(kind: TokenKind, len: usize) -> Self {
        Self { kind, len }
    }

    pub fn kind(&self) -> &TokenKind {
        &self.kind
    }

    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.len
    }
}
