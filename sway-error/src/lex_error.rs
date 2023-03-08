use sway_ast::token::{ClosingDelimiter, OpeningDelimiter};
use sway_types::{Ident, Span, Spanned};
use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq, Eq, PartialOrd, Hash)]
#[error("{}", kind)]
pub struct LexError {
    pub span: Span,
    pub kind: LexErrorKind,
}

#[derive(Error, Debug, Clone, PartialEq, Eq, PartialOrd, Hash)]
pub enum LexErrorKind {
    #[error("unclosed multiline comment")]
    UnclosedMultilineComment { unclosed_indices: Vec<usize> },
    #[error("unexpected close delimiter")]
    UnexpectedCloseDelimiter {
        position: usize,
        close_delimiter: ClosingDelimiter,
    },
    #[error("mismatched delimiters")]
    MismatchedDelimiters {
        open_position: usize,
        close_position: usize,
        open_delimiter: OpeningDelimiter,
        close_delimiter: ClosingDelimiter,
    },
    #[error("unclosed delimiter")]
    UnclosedDelimiter {
        open_position: usize,
        open_delimiter: OpeningDelimiter,
    },
    #[error("unclosed string literal")]
    UnclosedStringLiteral { position: usize },
    #[error("unclosed char literal")]
    UnclosedCharLiteral { position: usize },
    #[error("expected close quote")]
    ExpectedCloseQuote { position: usize },
    #[error("incomplete hex int literal")]
    IncompleteHexIntLiteral { position: usize },
    #[error("incomplete binary int literal")]
    IncompleteBinaryIntLiteral { position: usize },
    #[error("incomplete octal int literal")]
    IncompleteOctalIntLiteral { position: usize },
    #[error("invalid int suffix: {}", suffix)]
    InvalidIntSuffix { suffix: Ident },
    #[error("invalid character")]
    InvalidCharacter { position: usize, character: char },
    #[error("invalid hex escape")]
    InvalidHexEscape,
    #[error("unicode escape missing brace")]
    UnicodeEscapeMissingBrace { position: usize },
    #[error("invalid unicode escape digit")]
    InvalidUnicodeEscapeDigit { position: usize },
    #[error("unicode escape out of range")]
    UnicodeEscapeOutOfRange { position: usize },
    #[error("unicode escape represents an invalid char value")]
    UnicodeEscapeInvalidCharValue { span: Span },
    #[error("invalid escape code")]
    InvalidEscapeCode { position: usize },
}

impl Spanned for LexError {
    fn span(&self) -> Span {
        self.span.clone()
    }
}

impl LexError {
    pub fn span_ref(&self) -> &Span {
        &self.span
    }
}