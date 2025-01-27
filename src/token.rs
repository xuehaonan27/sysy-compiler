use logos::Logos;
use std::num::ParseIntError;

#[derive(Default, Debug, Clone, PartialEq)]
pub enum LexicalError {
    InvalidInteger(ParseIntError),
    #[default]
    InvalidToken,
}

impl From<ParseIntError> for LexicalError {
    fn from(err: ParseIntError) -> Self {
        LexicalError::InvalidInteger(err)
    }
}

#[derive(Logos, Debug, Clone, PartialEq)]
#[logos(extras = i32)]
pub enum Token {
    #[regex(r"[ \t\n\r]+", logos::skip)]
    Whitespace,
    #[regex(r"//[^\n\r]*[\n\r]*", logos::skip)]
    SingleLineComment,
    #[regex(r"/\*[^*]*\*+(?:[^/*][^*]*\*+)*/", logos::skip)]
    MultipleLinesComment,
    #[regex(r"[_a-zA-Z][_a-zA-Z0-9]*", |lex| lex.slice().to_string())]
    Ident(String),

    #[regex(r"[1-9][0-9]*", |lex| {
        let slice = lex.slice();
        i32::from_str_radix(slice, 10).ok()
    })]
    #[regex(r"0[0-7]*", |lex| {
        let slice = lex.slice();
        i32::from_str_radix(slice, 8).ok()
    })]
    #[regex(r"0[xX][0-9a-fA-F]+", |lex| {
        let slice = &lex.slice()[2..]; // strip "0x" or "0X"
        i32::from_str_radix(slice, 16).ok()
    })]
    IntConst(i32),

    #[token("const")]
    Const, // const
    #[token("if")]
    If, // if
    #[token("else")]
    Else, // else
    #[token("while")]
    While, // while
    #[token("break")]
    Break, // break
    #[token("continue")]
    Continue, // continue
    #[token("return")]
    Return, // return

    #[token("+")]
    Add, // +
    #[token("-")]
    Sub, // -
    #[token("*")]
    Mul, // *
    #[token("/")]
    Div, // /
    #[token("%")]
    Mod, // %
    #[token("^")]
    Pow, // ^
    #[token("!")]
    Not,

    #[token("&")]
    BitAnd, // &
    #[token("~")]
    BitNot, // ~
    #[token("|")]
    BitOr, // |
    #[token("<<")]
    ShiftL, // <<
    #[token(">>")]
    ShiftR, // >>

    #[token("&&")]
    And, // &&
    #[token("||")]
    Or, // ||

    #[token("==")]
    Eq, // ==
    #[token("!=")]
    Neq, // !=
    #[token("<=")]
    Le, // <=
    #[token(">=")]
    Ge, // >=
    #[token("<")]
    Lt, // <
    #[token(">")]
    Gt, // >
    #[token("=")]
    Assign, // =

    #[token("(")]
    LParen, // (
    #[token(")")]
    RParen, // )
    #[token("{")]
    LBrace, // {
    #[token("}")]
    RBrace, // }
    #[token("[")]
    LBracket, // [
    #[token("]")]
    RBracket, // ]

    #[token(",")]
    Comma, // ,
    #[token(";")]
    SemiColon, // ;
    #[token(":")]
    Colon, // :
    #[token("::")]
    DoubleColon, // ::
    #[token(".")]
    Dot, // .
    #[token("..")]
    Concat, // ..
    #[token("...")]
    Dots, // ...

    #[token("int")]
    Int, // int
    #[token("void")]
    Void, // void
}
