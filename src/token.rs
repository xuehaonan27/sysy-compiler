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
pub enum Token {
    #[regex(r"[ \t\n\r]+", logos::skip)]
    Whitespace,

    #[regex(r"[_a-zA-Z][_a-zA-Z0-9]*", |lex| lex.slice().to_string())]
    Ident(String),

    #[regex(r"[1-9][0-9]*", |lex| lex.slice().parse().ok())]
    DecimalConst(u64),

    #[regex(r"0[0-7]*", |lex| {
        let slice = lex.slice();
        u64::from_str_radix(slice, 8).ok()
    })]
    OctalConst(u64),

    #[regex(r"0[xX][0-9a-fA-F]+", |lex| {
        let slice = &lex.slice()[2..]; // strip "0x" or "0X"
        u64::from_str_radix(slice, 16).ok()
    })]
    HexadecimalConst(u64),

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
    LogicAnd, // &&
    #[token("||")]
    LogicOr, // ||

    #[token("==")]
    Equal, // ==
    #[token("!=")]
    NotEq, // !=
    #[token("<=")]
    LesEq, // <=
    #[token(">=")]
    GreEq, // >=
    #[token("<")]
    Less, // <
    #[token(">")]
    Greater, // >
    #[token("=")]
    Assign, // =

    #[token("(")]
    ParL, // (
    #[token(")")]
    ParR, // )
    #[token("{")]
    CurlyL, // {
    #[token("}")]
    CurlyR, // }
    #[token("[")]
    SqurL, // [
    #[token("]")]
    SqurR, // ]
    #[token(",")]
    Comma, // ,
    #[token(";")]
    SemiColon, // ;
    #[token(":")]
    Colon, // :
    #[token("::")]
    DoubColon, // ::
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
