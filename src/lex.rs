use std::{collections::{HashMap}, convert::TryFrom};

use crate::parse::CompilerErr;


#[derive(Debug, PartialEq, Clone)]
pub enum LexValue {
    None,
    Int(i64),
    Ident(String),
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub enum TokenKind {
    Int,
    Ident,
    Bang,
    Quote,
    Hashtag,
    Dollar,
    Percent,
    Ampersand,
    Apostrophe,
    LeftParen,
    RightParen,
    Asterisk,
    Plus,
    Comma,
    Hyphen,
    Period,
    Slash,
    Colon,
    SemiColon,
    Lt,
    Eq,
    Gt,
    Question,
    At,
    LeftBracket,
    BackSlash,
    RightBracket,
    Circumflex,
    LowLine,
    Grave,
    LeftBrace,
    VerticalBar,
    RightBrace,
    Tilde,
    // keywords
    Let,
    Fn,
    Return,
}

impl TryFrom<&char> for TokenKind {
    type Error = CompilerErr;

    fn try_from(value: &char) -> Result<Self, CompilerErr> {
        use TokenKind::*;
        match value {
            '!' => Ok(Bang),
            '"' => Ok(Quote),
            '#' => Ok(Hashtag),
            '$' => Ok(Dollar),
            '%' => Ok(Percent),
            '&' => Ok(Ampersand),
            '\''=> Ok(Apostrophe),
            '(' => Ok(LeftParen),
            ')' => Ok(RightParen),
            '*' => Ok(Asterisk),
            '+' => Ok(Plus),
            ',' => Ok(Comma),
            '-' => Ok(Hyphen),
            '.' => Ok(Period),
            '/' => Ok(Slash),
            ':' => Ok(Colon),
            ';' => Ok(SemiColon),
            '<' => Ok(Lt),
            '=' => Ok(Eq),
            '>' => Ok(Gt),
            '?' => Ok(Question),
            '@' => Ok(At),
            '[' => Ok(LeftBracket),
            '\\'=> Ok(BackSlash),
            ']' => Ok(RightBracket),
            '^' => Ok(Circumflex),
            '_' => Ok(LowLine),
            '`' => Ok(Grave),
            '{' => Ok(LeftBrace),
            '|' => Ok(VerticalBar),
            '}' => Ok(RightBrace),
            '~' => Ok(Tilde),
            _ => Err(CompilerErr::Unknown)
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
     pub pos: usize,
     pub kind: TokenKind,
     pub value: LexValue,
}

pub struct Lex<'a> {
    p: usize,
    code: &'a str,
    chars: Vec<char>,
    keywords: HashMap<String, TokenKind>,
    pub cur: Option<Token>,
    pub line: usize,
}

impl<'a> Lex<'a> {
    pub fn new(code: &'a str) -> Self {
        let mut l = Lex {
            p: 0,
            code: code,
            chars: code.chars().collect::<Vec<char>>(),
            keywords: HashMap::new(),
            cur: None,
            line: 1,
        };
        // add keywords here ⬇ and in the TokenKind list
        l.keywords.insert("let".into(), TokenKind::Let);
        l.keywords.insert("fn".into(), TokenKind::Fn);
        l.keywords.insert("return".into(), TokenKind::Return);
        l.next();
        l
    }

    pub fn next(&mut self) -> Option<Token> {
        'start: loop {
            let start = self.p;
            if let Some(c) = self.chars.get(self.p) {
                match c {
                    c if c.is_whitespace() => { 
                        if *c == '\n' { 
                            self.line += 1;
                        }
                        self.p += 1; 
                    },
                    'a'..='z' | 'A'..='Z' | '_' => {
                        while let Some(c) = self.chars.get(self.p) {
                            match c {
                                'a'..='z' | 'A'..='Z' | '_' | '0'..='9' => self.p += 1,
                                _ => break,
                            }
                        }
                        if let Some(key) = self.keywords.get(&self.code[start..self.p]) {
                            self.cur = Some(Token {
                                pos: start,
                                kind: *key,
                                value: LexValue::None,
                            });
                        } else {
                            self.cur = Some(Token {
                                pos: start,
                                kind: TokenKind::Ident,
                                value: LexValue::Ident(self.code[start..self.p].into())
                            });
                        }
                        break 'start;
                    },
                    '0'..='9' => {
                        while let Some(c) = self.chars.get(self.p) {
                            match c {
                                '0'..='9' => self.p += 1,
                                _ => break,
                            }
                        }
                        if let Ok(n) = self.code[start..self.p].parse::<i64>() {
                            self.cur = Some(Token {
                                pos: start,
                                kind: TokenKind::Int,
                                value: LexValue::Int(n)
                            });
                            break 'start;
                        }
                    },
                    // this gives us all the normal characters used in programming langauges
                    c if c.is_ascii_graphic() && !c.is_ascii_alphanumeric() => { 
                        if let Ok(tk) = TokenKind::try_from(c) {
                            self.p += 1; self.cur = Some(Token{pos: start, kind: tk, value: LexValue::None}); break;
                        } else {
                            // error
                        }
                    },
                    _ => { self.cur = None; break; },
                }
            } else {
                self.cur = None;
                break 'start;
            }
        }
        self.cur.clone()
    }

    pub fn peek(&mut self) -> Option<Token> {
        let p = self.p;
        let t = self.next();
        self.p = p;
        self.next();
        t
    }

    pub fn expect(&mut self, other: TokenKind) -> bool {
        let mut r = false;
        if let Some(t) = &self.cur {
            if t.kind == other { 
                r = true;
                self.next();
            } else {
                println!("Error expected `{:?}` got `{:?}` at line:{:}", other, t.kind, self.line);
            }
        } else {
            println!("Error expected `{:?}` but nothing's there!", other);
        } 
        r
    }

    pub fn maybe(&mut self, other: TokenKind) -> bool {
        let mut r = false;
        let tok = &self.cur;
        if let Some(t) = tok {
            if t.kind == other { 
                r = true;
                self.next();
            }
        }
        r
    }
}

#[test]
fn lex_num_test() {
    let l = Lex::new("1234 \n\n 1234");
    assert_eq!(l.cur, Some(Token{pos: 0, kind: TokenKind::Int, value: LexValue::Int(1234)}));
    assert_eq!(l.line, 1);
}

#[test]
fn lex_ident_test() {
    let mut l = Lex::new(" \n\n _he123llo_ 1234");
    assert_eq!(l.cur, Some(Token{pos: 4, kind: TokenKind::Ident, value: LexValue::Ident("_he123llo_".into())}));
    assert_eq!(l.line, 3);
    l.next();
    assert_eq!(l.cur, Some(Token{pos: 15, kind: TokenKind::Int, value: LexValue::Int(1234)}));
}

#[test]
fn lex_symbols_test() {
    let mut l = Lex::new("1 + 2 - 3");
    assert_eq!(l.cur, Some(Token{pos: 0, kind: TokenKind::Int, value: LexValue::Int(1)}));
    l.next();
    assert_eq!(l.cur, Some(Token{pos: 2, kind: TokenKind::Plus, value: LexValue::None}));
    l.next();
    assert_eq!(l.cur, Some(Token{pos: 4, kind: TokenKind::Int, value: LexValue::Int(2)}));
    l.next();
    assert_eq!(l.cur, Some(Token{pos: 6, kind: TokenKind::Hyphen, value: LexValue::None}));
    l.next();
    assert_eq!(l.cur, Some(Token{pos: 8, kind: TokenKind::Int, value: LexValue::Int(3)}));
}

#[test]
fn lex_expect_test() {
    let mut l = Lex::new(" \n\n id 1234 if ");
    assert_eq!(l.expect(TokenKind::Ident), true);
    assert_eq!(l.line, 3);
    assert_eq!(l.expect(TokenKind::Int), true);
    assert_eq!(l.expect(TokenKind::Int), false);
    l.next();
    assert_eq!(l.expect(TokenKind::Ident), false);
}

#[test]
fn lex_keyword_test() {
    let mut l = Lex::new(" \n\n ident let 1234");
    assert_eq!(l.cur, Some(Token{pos: 4, kind: TokenKind::Ident, value: LexValue::Ident("ident".into())}));
    assert_eq!(l.line, 3);
    l.next();
    assert_eq!(l.cur, Some(Token{pos: 10, kind: TokenKind::Let, value: LexValue::None}));
}