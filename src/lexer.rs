use std::fmt;
use std::str::FromStr as _;

use crate::word::Word;

const ALPHABET: &str = "aeijklmnopstuw";

#[derive(Debug, Clone, PartialEq, Eq)]
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

    Word(Word),
    Space(&'a str), // consecutive spaces
    Other(&'a str),
}

impl Token<'_> {
    #[must_use]
    pub fn is_other(&self) -> bool {
        matches!(self, Self::Other(..))
    }

    #[must_use]
    pub fn is_space(&self) -> bool {
        matches!(self, Self::Space(..))
    }
}

impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::LParen => f.write_str("("),
            Token::RParen => f.write_str(")"),
            Token::LBrack => f.write_str("["),
            Token::RBrack => f.write_str("]"),
            Token::LBrace => f.write_str("{"),
            Token::RBrace => f.write_str("}"),
            Token::Word(word) => f.write_str(word.as_unicode_str()),
            Token::Other(s) => f.write_str(s),
            Token::Space(s) => f.write_str(s),
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
    let leftover = input.trim_start_matches(|c| ALPHABET.contains(c));
    let count = input.len() - leftover.len();
    if count > 0 {
        let text = &input[..count];

        let token = Word::from_str(text)
            .map(Token::Word)
            .unwrap_or(Token::Other(text));

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
    ALPHABET.contains(c) || " ()[]{}".contains(c)
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
