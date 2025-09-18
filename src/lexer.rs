use std::str::FromStr as _;

use crate::word::Word;

const ALPHABET: &str = "aeijklmnopstuwAEIJKLMNOPSTUW";

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

    // consecutive spaces
    Space(usize),

    // everything else
    Other(&'a str),
}

// token is either a "sitelen Lasina" or something else
pub fn next_token(input: &'_ str) -> (Token<'_>, &'_ str) {
    // handle empty input
    if input.is_empty() {
        // TODO?: maybe it's better to use something like `Token::End`
        return (Token::Space(0), "");
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
        let token = Token::Space(count);
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
    ALPHABET.contains(c) || " ()[]{}+-_.:".contains(c)
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
