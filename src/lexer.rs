use std::str::FromStr as _;

use crate::show;
use crate::word::Word;

const ALPHABET: &str = "aeijklmnopstuw";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Token<'a> {
    // ()
    LParen,
    RParen,

    // []
    LBrack,
    RBrack,

    // {}
    LBrace,
    RBrace,

    // +
    Plus,

    // -
    Minus,

    // _
    Underscore,

    // .
    Dot,

    // :
    Colon,

    // valid sitelen Lasina word
    Word(Word),

    // non sitelen Lasina word but uses all alphabetical letters
    Lasina(&'a str),

    // consecutive spaces
    Space(&'a str),

    // everything else
    Other(&'a str),
}

impl<'a> Token<'a> {
    pub fn from_sitelen(c: char) -> Option<Self> {
        show::TOKEN_MODIFIER
            .iter()
            .find(|(_, m)| m.as_sitelen() == c)
            .map(|(t, _)| *t)
            .or_else(|| Word::from_sitelen(c).map(Self::Word))
    }

    pub fn as_literal(&self) -> &'a str {
        match self {
            Token::LParen => "(",
            Token::RParen => ")",
            Token::LBrack => "[",
            Token::RBrack => "]",
            Token::LBrace => "{",
            Token::RBrace => "}",
            Token::Plus => "+",
            Token::Minus => "-",
            Token::Underscore => "_",
            Token::Dot => ".",
            Token::Colon => ":",
            Token::Word(word) => word.as_lasina(),
            Token::Space(spaces) => spaces,
            Token::Lasina(word) => word,
            Token::Other(other) => other,
        }
    }
}

// token is either a "sitelen Lasina" or something else
pub fn next_token(input: &'_ str) -> (Token<'_>, &'_ str) {
    // handle empty input
    if input.is_empty() {
        // TODO?: maybe it's better to use something like `Token::End`
        return (Token::Space(""), "");
    }

    // parse single character modifier
    let mut iter = input.chars();
    let first = iter.next().unwrap_or_default();
    let leftover = iter.as_str();
    let token = 'token: {
        Some(match first {
            '(' => Token::LParen,
            ')' => Token::RParen,
            '[' => Token::LBrack,
            ']' => Token::RBrack,
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            '+' => Token::Plus,
            '-' => Token::Minus,
            '_' => Token::Underscore,
            '.' => Token::Dot,
            ':' => Token::Colon,
            _ => break 'token None,
        })
    };

    if let Some(token) = token {
        return (token, leftover);
    }

    // parse space
    let leftover = input.trim_start_matches(' ');
    let count = input.len() - leftover.len();
    if count > 0 {
        let token = Token::Space(&input[..count]);
        return (token, leftover);
    }

    // parse word
    let leftover = input.trim_start_matches(|c: char| c.is_alphabetic());
    let count = input.len() - leftover.len();
    if count > 0 {
        let text = &input[..count];

        let token = Word::from_str(text)
            .map(Token::Word)
            .unwrap_or_else(|_| {
                if text.chars().all(|c| ALPHABET.contains(c.to_ascii_lowercase())) {
                    Token::Lasina(text)
                } else {
                    Token::Other(text)
                }
            });

        return (token, leftover);
    }

    // consume until next valid character
    let leftover = input.trim_start_matches(|c| !valid_char_token(c));
    let count = input.len() - leftover.len();
    let text = &input[..count];
    let token = Token::Other(text);

    (token, leftover)
}

fn valid_char_token(c: char) -> bool {
    c.is_alphabetic() || " ()[]{}+-_.:".contains(c)
}

pub fn tokens(mut input: &'_ str) -> impl Iterator<Item = Token<'_>> {
    std::iter::from_fn(move || {
        if input.is_empty() {
            return None;
        }

        let token;
        (token, input) = next_token(input);
        Some(token)
    })
}
